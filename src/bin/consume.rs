#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

#[derive(Parser)]
struct Orgs {
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let Orgs { files } = Orgs::parse();
    for file in files {
        print!("{}", fs::read_to_string(&file).unwrap());
        fs::remove_file(file).unwrap();
    }
    Ok(())
}
