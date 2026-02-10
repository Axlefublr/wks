#![allow(unused_variables)]
#![allow(dead_code)]

use rand::seq::IteratorRandom;
use wks::prelude::*;

#[derive(Parser)]
struct Octopus {
    #[arg(short = 'n', long)]
    lines: Option<Consideration>,
    cache_name: String,
    consider: Consideration,
    path: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug)]
enum Consideration {
    Number(usize),
    Percentage(u8),
}

impl FromStr for Consideration {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(number) = s.parse() {
            Ok(Self::Number(number))
        } else {
            Ok(Self::Percentage(s[..(s.len() - 1)].parse()?))
        }
    }
}

fn main() -> anyhow::Result<()> {
    let octopus = Octopus::parse();
    let mut carpholder = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(false)
        .create(true)
        .open({
            let mut the = dirs::cache_dir().unwrap();
            the.push("propose.rs.jsonc");
            the
        })
        .unwrap();
    let mut buf = String::new();
    carpholder
        .read_to_string(&mut buf)
        .unwrap();
    let mut carp: HashMap<String, Vec<String>> = serde_json::from_str(&buf).unwrap_or_default();
    let carplet = carp
        .entry(octopus.cache_name)
        .or_default(); // carpet made of carps
    let mut rng = rand::rng();
    let contents = if let Some(path) = octopus.path {
        let mut input_file = OpenOptions::new()
            .read(true)
            .create(false)
            .open(path)
            .context("input file doesn't exist")?;
        let mut contents = String::new();
        input_file
            .read_to_string(&mut contents)
            .unwrap();
        contents
    } else {
        let mut contents = String::new();
        io::stdin()
            .read_to_string(&mut contents)
            .unwrap();
        contents
    };
    let lines = contents.lines();
    let line_count = lines.clone().count();
    let starting_index = match octopus.consider {
        Consideration::Number(num) => carplet.len().saturating_sub(num),
        Consideration::Percentage(percentage) => carplet
            .len()
            .saturating_sub(line_count * percentage as usize / 100),
    };
    let pickable_lines = lines.filter(|&line| !carplet[starting_index..].contains(&line.to_owned()));
    let picked_lines = pickable_lines
        .clone()
        .by_ref()
        .sample(
            &mut rng,
            match octopus
                .lines
                .unwrap_or(Consideration::Number(1))
            {
                Consideration::Number(num) => num,
                Consideration::Percentage(perc) => pickable_lines.count() * perc as usize / 100,
            },
        );
    if picked_lines.is_empty().not() {
        println!("{}", picked_lines.join("\n"));
        carplet.append(
            &mut picked_lines
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
        );
    }
    let carplet_len = carplet.len();
    if carplet_len > line_count {
        carplet.drain(..(carplet_len - line_count));
    }
    let prac = serde_json::to_string_pretty(&carp).unwrap();
    carpholder.set_len(0).unwrap();
    carpholder.rewind().unwrap();
    carpholder
        .write_all(prac.as_bytes())
        .unwrap();
    Ok(())
}
