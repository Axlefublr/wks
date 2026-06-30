#![allow(unused_variables)]
#![allow(dead_code)]

use ignore::DirEntry;
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
    walk(&cwd, &cwd);
    Ok(())
}

fn entries(cwd: &Path) -> Vec<DirEntry> {
    let mut walk_builder = WalkBuilder::new(cwd);
    walk_builder
        .hidden(false)
        .parents(true)
        .follow_links(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .add_custom_ignore_filename(".helix/ignore")
        .ignore(true)
        .max_depth(Some(1))
        .sort_by_file_name(|name1, name2| name1.cmp(name2))
        .build()
        .filter_map(Result::ok)
        .filter(|the| the.path() != cwd)
        .collect::<Vec<_>>()
}

fn walk(root_cwd: &Path, cwd: &Path) {
    let mut entries = entries(cwd);
    entries.sort_by_key(|the| the.file_type().unwrap().is_dir());
    let directories = entries
        .into_iter()
        .filter(|entry| {
            let is_dir = entry.file_type().unwrap().is_dir();
            let path = entry.path();
            let the = path
                .strip_prefix(root_cwd)
                .unwrap()
                .display();
            println!("{}{}", the, if is_dir { "/" } else { "" });
            is_dir
        })
        .collect::<Vec<_>>();
    for entry in directories {
        walk(root_cwd, entry.path())
    }
}
