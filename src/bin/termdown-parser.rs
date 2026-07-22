#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

#[derive(Parser)]
struct Octopus {
    timestamps: Vec<String>,
}

#[derive(Debug)]
enum Timestamp {
    Resolvable(Resolution),
    Unresolvable(String),
}

#[derive(Debug)]
enum Resolution {
    Later(String),
    Resolved(DateTime<Local>),
}

fn main() -> Result<()> {
    let Octopus { timestamps } = Octopus::parse();
    let mut first_pass = Vec::new();
    for arg in timestamps.into_iter() {
        if arg.starts_with(['-', '+']) {
            first_pass.push(Timestamp::Resolvable(Resolution::Later(arg)));
        } else if arg.contains(':') {
            first_pass.push(Timestamp::Resolvable(Resolution::Resolved(Local::now())));
        } else {
            first_pass.push(Timestamp::Unresolvable(arg));
        }
    }
    println!("{first_pass:?}");
    Ok(())
}
