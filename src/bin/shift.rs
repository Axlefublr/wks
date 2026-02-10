#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let path: PathBuf = args
        .next()
        .and_then(|maybe_path| maybe_path.parse().ok())
        .expect("argument is not a filepath");
    let count: usize = args
        .next()
        .and_then(|maybe_count| maybe_count.parse().ok())
        .unwrap_or(1);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("read file");
    let mut lines = contents.lines();
    for line in lines.by_ref().take(count) {
        println!("{}", line);
    }
    file.set_len(0).expect("set_len");
    file.rewind().expect("rewind");
    let lines = lines.collect::<Vec<&str>>();
    if lines.is_empty() {
        return Ok(());
    }
    writeln!(file, "{}", lines.join("\n")).expect("write lines back");
    Ok(())
}
