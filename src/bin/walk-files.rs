#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

use ignore::WalkBuilder;

fn main() -> Result<()> {
    let cwd = env::args()
        .nth(1)
        .map(|the| PathBuf::from_str(&the).expect("passed directory is a path"))
        .unwrap_or_else(|| {
            env::current_dir().expect("assumed current directory because directory not passed")
        })
        .canonicalize()
        .expect("canonicalize resulting path");
    let mut walk_builder = WalkBuilder::new(&cwd);
    let mut entries = walk_builder
        .hidden(false)
        .parents(true)
        .follow_links(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .add_custom_ignore_filename(".helix/ignore")
        .ignore(true)
        .sort_by_file_name(|name1, name2| name1.cmp(name2))
        .build()
        .filter_map(Result::ok)
        .filter(|the| the.path() != cwd)
        .collect::<Vec<_>>();
    entries.sort_by_key(|the| the.file_type().unwrap().is_dir());
    entries.sort_by_key(|the| the.depth());
    for entry in entries {
        let is_dir = entry.file_type().unwrap().is_dir();
        let the = entry
            .path()
            .strip_prefix(&cwd)
            .unwrap()
            .display();
        println!("{}{}", the, if is_dir { "/" } else { "" });
    }
    Ok(())
}
