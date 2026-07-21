#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

#[derive(Parser)]
struct Octopus {
    path: PathBuf,
    new_name: String,
    new_url: String,
}

fn main() -> Result<()> {
    let Octopus {
        path,
        new_name,
        new_url,
    } = Octopus::parse();
    // let path = PathBuf::from("/home/axlefublr/.local/share/magazine/l");
    // let new_name = String::from("discord server mine");
    // let new_url = String::from("https://discord.gg/bgVSg362dK");
    let text = fs::read_to_string(&path)?;
    let mut appeared = false;
    let mut buf = String::new();
    for (index, line) in text.lines().enumerate() {
        let mut the = line.split(" — ");
        let name = the.next().unwrap();
        let url = the.next().unwrap();
        if name == new_name {
            appeared = true;
            buf.push_str(name);
            buf.push_str(" — ");
            buf.push_str(&new_url);
        } else if url == new_url {
            println!("{}", index + 1);
            return Ok(());
        } else {
            buf.push_str(line);
        }
        buf.push('\n');
    }
    if !appeared {
        buf.push_str(&new_name);
        buf.push_str(" — ");
        buf.push_str(&new_url);
        buf.push('\n');
    } else {
        Command::new("notify-send")
            .arg("name duplicate overwritten")
            .status()
            .unwrap();
    }
    fs::write(path, buf.as_bytes())?;
    Ok(())
}
