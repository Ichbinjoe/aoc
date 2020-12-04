extern crate regex;
use crate::futil::read_lines;
use anyhow::anyhow;
use std::path::PathBuf;

use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref HEIGHT_RE: regex::Regex = regex::Regex::new("^(\\d+)(in|cm)$").unwrap();
    static ref HAIR_RE: regex::Regex = regex::Regex::new("^#[0-9a-f]{6}$").unwrap();
    static ref EYE_RE: regex::Regex = regex::Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_RE: regex::Regex = regex::Regex::new("^\\d{9}$").unwrap();
}

struct Passport {
    p: HashMap<String, String>,
}

fn exists_in_range<T: AsRef<str>>(s: T, a: usize, b: usize) -> bool {
    s.as_ref()
        .parse::<usize>()
        .ok()
        .map(|v| v >= a && v <= b)
        .unwrap_or(false)
}

fn height_is(s: Option<&String>) -> bool {
    s.and_then(|v| HEIGHT_RE.captures(v))
        .and_then(|caps| caps.get(1).zip(caps.get(2)))
        .and_then(|(c1, c2)| {
            let unit = c2.as_str();
            if unit == "cm" {
                Some(exists_in_range(c1.as_str(), 150, 193))
            } else if unit == "in" {
                Some(exists_in_range(c1.as_str(), 59, 76))
            } else {
                None
            }
        })
        .unwrap_or(false)
}

fn matches_re(s: Option<&String>, re: &regex::Regex) -> bool {
    s.and_then(|a| Some(re.is_match(a))).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byr() {
        assert!(exists_in_range("2002", 1920, 2002));
        assert!(!exists_in_range("2003", 1920, 2002));
    }

    #[test]
    fn test_hgt() {
        let a = "60in".to_string();
        let b = "190cm".to_string();
        let c = "190in".to_string();
        let d = "190".to_string();
        assert!(height_is(Some(&a)));
        assert!(height_is(Some(&b)));
        assert!(!height_is(Some(&c)));
        assert!(!height_is(Some(&d)));
        assert!(!height_is(None));
    }

    #[test]
    fn test_hcl() {
        let a = "#123abc".to_string();
        let b = "#123abz".to_string();
        let c = "123abc".to_string();

        assert!(matches_re(Some(&a), &HAIR_RE));
        assert!(!matches_re(Some(&b), &HAIR_RE));
        assert!(!matches_re(Some(&c), &HAIR_RE));
        assert!(!matches_re(None, &HAIR_RE));
    }

    #[test]
    fn test_ecl() {
        let a = "brn".to_string();
        let b = "wat".to_string();

        assert!(matches_re(Some(&a), &EYE_RE));
        assert!(!matches_re(Some(&b), &EYE_RE));
    }

    #[test]
    fn test_pid() {
        let a = "000000001".to_string();
        let b = "0123456789".to_string();

        assert!(matches_re(Some(&a), &PID_RE));
        assert!(!matches_re(Some(&b), &PID_RE));
    }
}

impl Passport {
    fn field_must<F>(&self, s: &str, f: F) -> bool
    where
        F: FnOnce(&String) -> bool,
    {
        self.p.get(s).map(f).unwrap_or(false)
    }

    fn valid(&self) -> bool {
        self.field_must("byr", |a| exists_in_range(a, 1920, 2002))
            && self.field_must("iyr", |a| exists_in_range(a, 2010, 2020))
            && self.field_must("eyr", |a| exists_in_range(a, 2020, 2030))
            && height_is(self.p.get("hgt"))
            && matches_re(self.p.get("hcl"), &HAIR_RE)
            && matches_re(self.p.get("ecl"), &EYE_RE)
            && matches_re(self.p.get("pid"), &PID_RE)
            && (self.p.len() == 7 || (self.p.len() == 8 && self.p.contains_key("cid")))
    }

    fn insert(&mut self, frag: &str) {
        let mut fragparts = frag.trim().split(":");
        let k = fragparts.next().unwrap();
        let v = fragparts.next().unwrap();
        self.p.insert(k.to_string(), v.to_string());
    }

    fn new() -> Passport {
        Passport { p: HashMap::new() }
    }
}

pub fn y2020p4(input: &PathBuf) -> Result<(), anyhow::Error> {
    let mut passport = Passport::new();

    let mut valids = 0;
    for maybe_line in read_lines(input)? {
        let line = maybe_line?;

        if line.len() == 0 {
            if passport.valid() {
                valids += 1;
            }
            passport = Passport::new();
            continue;
        }

        let fragments = line.split(" ");
        for fragment in fragments {
            passport.insert(fragment);
        }
    }

    if passport.valid() {
        valids += 1
    }

    println!("valids {}", valids);

    Ok(())
}
