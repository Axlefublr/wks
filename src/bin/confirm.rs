#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

#[derive(Parser)]
struct Tuna {
    question: String,
    alternatives: Vec<Alternative>,
}

#[derive(Clone, Debug)]
struct Alternative {
    text: String,
    shortcut: char,
}

impl FromStr for Alternative {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let text = s.to_owned();
        let shortcut = s
            .chars()
            .skip_while(|&chr| chr != '[')
            .skip(1)
            .take(1)
            .next()
            .context("no shortcut in alternative")?;
        Ok(Alternative { text, shortcut })
    }
}

fn main() -> Result<()> {
    let tuna = Tuna::parse();
    eprintln!("{}", tuna.question);
    let mut valid_shortcuts = HashSet::new();
    let alternatives = tuna
        .alternatives
        .into_iter()
        .map(|Alternative { text, shortcut }| {
            valid_shortcuts.insert(shortcut);
            text
        })
        .collect::<Vec<_>>()
        .join(" / ");
    eprint!("{}: ", alternatives);
    io::stdout()
        .lock()
        .flush()
        .unwrap();
    let term = console::Term::stderr();
    loop {
        let taken_char = term.read_char().unwrap();
        if valid_shortcuts.contains(&taken_char) {
            println!("{}", taken_char);
            break;
        }
    }
    eprintln!();
    Ok(())
}
