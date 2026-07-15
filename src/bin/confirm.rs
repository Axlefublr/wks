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
    shortcut: Option<char>,
}

impl FromStr for Alternative {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let text = s.to_owned();
        let Some(shortcut) = s
            .chars()
            .skip_while(|&chr| chr != '[')
            .skip(1)
            .take(1)
            .next()
        else {
            return Ok(Alternative {
                text: Default::default(),
                shortcut: None,
            });
        };
        Ok(Alternative {
            text,
            shortcut: Some(shortcut),
        })
    }
}

fn main() -> Result<()> {
    let tuna = Tuna::parse();
    if tuna.question.is_empty().not() {
        eprintln!("{}", tuna.question);
    }
    let mut valid_shortcuts = HashSet::new();
    let mut alternatives = String::new();
    let mut previous_was_newline = true;
    for Alternative { text, shortcut } in tuna.alternatives.into_iter() {
        if let Some(shortcut) = shortcut {
            valid_shortcuts.insert(shortcut);
            if !previous_was_newline {
                alternatives.push(' ');
            }
            previous_was_newline = false;
            alternatives.push_str(&text);
        } else {
            alternatives.push('\n');
            previous_was_newline = true;
        }
    }
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
            eprintln!();
            break;
        }
    }
    Ok(())
}
