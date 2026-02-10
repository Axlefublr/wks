#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    let mut sorting = false;
    let mut collected = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("[[sort on]]") {
            println!("{}", line);
            sorting = true;
        } else if line.contains("[[sort off]]") {
            sorting = false;
            collected.sort();
            println!("{}", collected.join("\n"));
            collected.truncate(0);
            println!("{}", line);
        } else if sorting {
            collected.push(line);
        } else {
            println!("{}", line);
        }
    }
    if !collected.is_empty() {
        collected.sort();
        println!("{}", collected.join("\n"));
    }
    Ok(())
}
