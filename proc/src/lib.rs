extern crate proc_macro;
extern crate quote;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream, Peek};
use syn::{parse_macro_input, Ident, LitInt, Result, Token};

struct IntcodeOpInvocation {
    modecode: Ident,
    op: Ident,
    args: usize,
}

fn must_parse<L: Peek, T: Parse>(token: L, stream: &ParseStream) -> Result<T> {
    let lookahead = stream.lookahead1();
    if !lookahead.peek(token) {
        Err(lookahead.error())
    } else {
        stream.parse()
    }
}

impl Parse for IntcodeOpInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let modecode_ident = must_parse(Ident, &input)?;
        let _: Token!(,) = must_parse(Token!(,), &input)?;
        let op_ident = must_parse(Ident, &input)?;
        let _: Token!(,) = must_parse(Token!(,), &input)?;
        let args: LitInt = must_parse(LitInt, &input)?;
        let arg_count = args.base10_parse::<usize>()?;

        Ok(IntcodeOpInvocation {
            modecode: modecode_ident,
            op: op_ident,
            args: arg_count,
        })
    }
}

#[proc_macro]
pub fn intcode_op(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as IntcodeOpInvocation);

    let modecode = input.modecode;
    let op = input.op;
    let args = input.args;

    let mut arglist = Vec::new();
    for i in 0..args {
        let ident = Ident::new(&format!("arg{}", i), Span::call_site());
        arglist.push(ident);
    }

    let mut assignlist = Vec::new();
    for i in 0..args {
        let ident = arglist.get(i as usize).unwrap();
        let v = quote!{*unsafe{self.data.get_unchecked(self.ip + 1 + #i)}};
        assignlist.push(quote! {let #ident =
            match #modecode % 10 {
                0 => OpArg::Position(#v),
                1 => OpArg::Immediate(#v),
                2 => OpArg::Relative(#v),
                _ => return Err(IntcodeError::IllegalOpcode(*instruction))
            };
        });
        if i != args - 1 {
            assignlist.push(quote!{#modecode /= 10;});
        }
    }

    let expanded = quote! {
        if #args > 0 && self.data.len() < self.ip + 1 + #args {
            Err(IntcodeError::OutOfBoundsArguments((self.ip + 1 + #args) as i32))
        } else {
            #(#assignlist);*;
            let step_result = #op(self, #(#arglist),*)?;
            if step_result == StepResult::Continue {
                self.ip += 1 + #args;
            }
            Ok(step_result)
        }
    };

    TokenStream::from(expanded)
}
