use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_csints<P>(filename: P) -> Result<Vec<i64>, anyhow::Error>
where
    P: AsRef<Path>,
{
    let mut result = Vec::new();
    let contents = std::fs::read_to_string(filename)?;
    for num in contents.split(",") {
        result.push(num.trim().parse::<i64>()?);
    }

    Ok(result)
}
