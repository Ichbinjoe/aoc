extern crate proc_macro;
extern crate quote;

use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream, Peek};
use syn::{parse_macro_input, Ident, LitInt, Result, Token};

struct IntcodeOpInvocation {
    base: i64,
    op: Ident,
    in_args: usize,
    out_args: usize,
}

struct MultiIntcodeOpInvocations {
    ops: Vec<IntcodeOpInvocation>,
    instruction: Ident,
}

fn must_parse<L: Peek, T: Parse>(token: L, stream: &ParseStream) -> Result<T> {
    let lookahead = stream.lookahead1();
    if !lookahead.peek(token) {
        Err(lookahead.error())
    } else {
        stream.parse()
    }
}

impl Parse for MultiIntcodeOpInvocations {
    fn parse(input: ParseStream) -> Result<Self> {
        let n: LitInt = must_parse(LitInt, &input)?;
        let n_n = n.base10_parse::<usize>()?;
        let _: Token!(,) = must_parse(Token!(,), &input)?;
        let instruction_ident = must_parse(Ident, &input)?;
        let _: Token!(,) = must_parse(Token!(,), &input)?;
        let mut r = MultiIntcodeOpInvocations {
            ops: Vec::new(),
            instruction: instruction_ident,
        };

        for i in 0..n_n {
            r.ops.push(IntcodeOpInvocation::parse(&input)?);
            if i != n_n - 1 {
                let _: Token!(,) = must_parse(Token!(,), &input)?;
            }
        }
        Ok(r)
    }
}

impl Parse for IntcodeOpInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let base: LitInt = must_parse(LitInt, &input)?;
        let _: Token!(,) = must_parse(Token!(,), &input)?;
        let op_ident = must_parse(Ident, &input)?;
        let _: Token!(,) = must_parse(Token!(,), &input)?;
        let in_args: LitInt = must_parse(LitInt, &input)?;
        let _: Token!(,) = must_parse(Token!(,), &input)?;
        let out_args: LitInt = must_parse(LitInt, &input)?;
        let base_i = base.base10_parse::<i64>()?;
        let in_arg_count = in_args.base10_parse::<usize>()?;
        let out_arg_count = out_args.base10_parse::<usize>()?;

        Ok(IntcodeOpInvocation {
            base: base_i,
            op: op_ident,
            in_args: in_arg_count,
            out_args: out_arg_count,
        })
    }
}

fn things<T: Clone>(a: &Vec<T>, l: usize) -> Vec<Vec<T>> {
    // Okay, get ready for some CHEESE
    if l == 0 {
        vec![vec![]]
    } else {
        let mut r = Vec::new();
        let q = things(&a, l - 1);
        for v in a {
            for qq in &q {
                let mut qqq = qq.clone();
                qqq.push(v.clone());
                r.push(qqq);
            }
        }
        r
    }
}

#[proc_macro]
pub fn intcode_op(input: TokenStream) -> TokenStream {
    let all_inputs = parse_macro_input!(input as MultiIntcodeOpInvocations);

    let mut entries = vec![];
    for input in all_inputs.ops {
        let base = input.base;
        let op = input.op;
        let in_args = input.in_args;
        let out_args = input.out_args;

        let in_opts = vec![
            (0, quote! {PositionInput}),
            (1, quote! {ImmediateInput}),
            (2, quote! {RelativeInput}),
        ];
        let out_opts = vec![(0, quote! {PositionOutput}), (2, quote! {RelativeOutput})];

        let arguments: Vec<Ident> = (0..in_args + out_args)
            .map(|i| Ident::new(&format!("arg{}", i), Span::call_site()))
            .collect();
        for input_combo in things(&in_opts, in_args) {
            let mut input_only_code = base;
            let mut input_pow = 100;
            for (id, _) in &input_combo {
                input_only_code += id * input_pow;
                input_pow *= 10;
            }
            for output_combo in things(&out_opts, out_args) {
                let mut code = input_only_code;
                let mut output_pow = input_pow;
                for (id, _) in &output_combo {
                    code += id * output_pow;
                    output_pow *= 10;
                }

                let arg_types = input_combo
                    .iter()
                    .chain(output_combo.iter())
                    .map(|(_, arg_type)| arg_type);

                let assign_list: Vec<proc_macro2::TokenStream> = arguments
                    .iter()
                    .enumerate()
                    .zip(arg_types)
                    .map(|((i, arg), t)| {
                        quote! {
                            let #arg = #t::from(*unsafe{self.data.get_unchecked(self.ip + 1 + #i)})
                        }
                    })
                    .collect();

                let args = in_args + out_args;
                let entry = quote! {
                    #code => {
                        if #args > 0 && self.data.len() < self.ip + 1 + #args {
                            Err(IntcodeError::OutOfBoundsArguments((self.ip + 1 + #args) as i64))
                        } else {
                            #(#assign_list);*;
                            let step_result = #op(self, #(#arguments),*)?;
                            if step_result == StepResult::Continue {
                                self.ip += 1 + #args;
                            }
                            Ok(step_result)
                        }
                    }
                };

                entries.push(entry);
            }
        }
    }

    let instruction = all_inputs.instruction;

    TokenStream::from(quote! {
        match *#instruction {
            #(#entries),*
            _ => Err(IntcodeError::IllegalOpcode(*#instruction)),
        }
    })
}
