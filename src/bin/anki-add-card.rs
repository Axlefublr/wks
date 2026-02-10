#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

const VALID_DECKS: [&str; 2] = ["Once", "Freq"];
const VALID_NOTE_TYPES: [&str; 5] = ["b", "d", "f", "fb", "h"];

fn main() -> Result<()> {
    let contents = fs::read_to_string("/home/axlefublr/.cache/mine/anki-card.html").expect("read card file");
    let mut lines = contents.lines().map(str::trim);

    let deck = lines.next().unwrap();
    ensure!(!VALID_DECKS.contains(&deck), "invalid deck");
    let note_type = lines.next().unwrap();
    ensure!(!VALID_NOTE_TYPES.contains(&note_type), "invalid note type");

    let mut lines: Vec<_> = lines.collect();
    lines.insert(0, note_type);
    lines.insert(0, deck);

    let fields = lines.len();
    ensure!(
        !(4..=6).contains(&fields),
        "expected 4..=6 fields, got {}",
        fields
    );

    lines.resize(6, "");

    let card = lines
        .into_iter()
        .map(|line| format!("\"{}\"", line.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join("|");
    println!("{}", card);
    Ok(())
}
