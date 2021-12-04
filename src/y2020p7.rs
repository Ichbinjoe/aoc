use crate::futil::read_lines;
use std::path::PathBuf;

use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
struct BagIdentifier {
    attr: String,
    color: String,
}

impl BagIdentifier {
    fn take_ident<'a, T: Iterator<Item = &'a str>>(i: &mut T) -> Option<BagIdentifier> {
        i.next().and_then(|attr| {
            i.next().map(|color| BagIdentifier {
                attr: attr.to_owned(),
                color: color.to_owned(),
            })
        })
    }
}

type BagRegistry = HashMap<BagIdentifier, BagDefinition>;

struct BagDefinition {
    contents: Vec<(u32, BagIdentifier)>,
}

impl BagDefinition {
    fn take_bag_contents<'a, T: Iterator<Item = &'a str>>(
        i: &mut T,
    ) -> Option<(u32, BagIdentifier)> {
        i.next()
            .and_then(|quantity| quantity.parse::<u32>().ok())
            .zip(BagIdentifier::take_ident(i))
            // take 'bags'
            .zip(i.next())
            .map(|(v, _)| v)
    }

    fn take_contents<'a, T: Iterator<Item = &'a str>>(i: &mut T) -> BagDefinition {
        let mut bd = BagDefinition {
            contents: Vec::new(),
        };

        while let Some(bc) = BagDefinition::take_bag_contents(i) {
            bd.contents.push(bc);
        }
        bd
    }

    fn contains_bag(&self, bid: &BagIdentifier, br: &BagRegistry) -> bool {
        for (_, t) in &self.contents {
            if t == bid {
                return true;
            }
            if let Some(contents) = br.get(&t) {
                if contents.contains_bag(bid, br) {
                    return true;
                }
            }
        }

        return false;
    }

    fn num_contained_bags(&self, br: &BagRegistry) -> u32 {
        let mut count = 0;
        for (bcount, bid) in &self.contents {
            let contents = br.get(&bid).unwrap();
            count += bcount * (contents.num_contained_bags(br) + 1);
        }

        return count;
    }
}

pub fn y2020p7(input: &PathBuf) -> Result<(), anyhow::Error> {
    let mut registry = HashMap::<BagIdentifier, BagDefinition>::new();

    for maybe_line in read_lines(input)? {
        let line = maybe_line?;
        let mut parts = line.split(" ");

        let id = BagIdentifier::take_ident(&mut parts).unwrap();
        parts.next(); // bags
        parts.next(); // contain
        let contents = BagDefinition::take_contents(&mut parts);

        registry.insert(id, contents);
    }

    let shiny_gold_bag = BagIdentifier {
        attr: "shiny".to_owned(),
        color: "gold".to_owned(),
    };

    let shiny = registry
        .values()
        .filter(|contents| contents.contains_bag(&shiny_gold_bag, &registry))
        .count();
    let num_contained = registry
        .get(&shiny_gold_bag)
        .unwrap()
        .num_contained_bags(&registry);
    println!("shiny: {}, contained: {}", shiny, num_contained);
    Ok(())
}

#[cfg(test)]
mod tests {}
