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

fn passport_valid(m: &HashMap<String, String>) -> bool {
    m.contains_key("byr")
        && m.contains_key("iyr")
        && m.contains_key("eyr")
        && m.contains_key("hgt")
        && m.contains_key("hcl")
        && m.contains_key("ecl")
        && m.contains_key("pid")
        && (m.len() == 7 || (m.len() == 8 && m.contains_key("cid")))
    //m.contains_key("cid")
}

struct Passport {
    p: HashMap<String, String>,
}

fn exists_in_range2(s: Option<String>, a: usize, b: usize) -> bool {
    s.and_then(|v| v.parse::<usize>().ok())
        .and_then(|v| Some(v >= a && v <= b))
        .unwrap_or(false)
}

fn exists_in_range(s: Option<&String>, a: usize, b: usize) -> bool {
    s.and_then(|v| v.parse::<usize>().ok())
        .and_then(|v| Some(v >= a && v <= b))
        .unwrap_or(false)
}

fn height_is(s: Option<&String>) -> bool {
    s.and_then(|v| HEIGHT_RE.captures(v))
        .and_then(|caps| {
            let cap1 = caps.get(1).and_then(|a| Some(a.as_str().to_string()));
            let cap2 = caps.get(2).and_then(|a| Some(a.as_str().to_string()));
            if cap1 == None || cap2 == None {
                None
            } else {
                let unit = cap2.unwrap();
                if unit == "cm" {
                    Some(exists_in_range2(cap1, 150, 193))
                } else if unit == "in" {
                    Some(exists_in_range2(cap1, 59, 76))
                } else {
                    None
                }
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
        let a = "2002".to_string();
        assert!(exists_in_range(Some(&a), 1920, 2002));
        let b = "2003".to_string();
        assert!(!exists_in_range(Some(&b), 1920, 2002));
        assert!(!exists_in_range(None, 1, 12));
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
    fn valid(&self) -> bool {
        exists_in_range(self.p.get("byr"), 1920, 2002)
            && exists_in_range(self.p.get("iyr"), 2010, 2020)
            && exists_in_range(self.p.get("eyr"), 2020, 2030)
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

        if line.trim().len() == 0 {
            // PASSPORT
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
