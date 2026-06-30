#![allow(unused_variables)]
#![allow(dead_code)]

use ignore::DirEntry;
use ignore::WalkBuilder;
use wks::prelude::*;

fn main() -> Result<()> {
    let cwd = env::args()
        .nth(1)
        .map(|the| PathBuf::from_str(&the).expect("passed directory is a path"))
        .unwrap_or_else(|| {
            env::current_dir().expect("assumed current directory because directory not passed")
        })
        .canonicalize()
        .expect("canonicalize resulting path");
    let mut queue = VecDeque::new();
    queue.push_back(cwd.clone());

    while let Some(dir) = queue.pop_front() {
        let mut entries = entries(&dir);
        entries.sort_by_key(|the| the.file_type().unwrap().is_dir());

        let dirs = entries
            .into_iter()
            .filter(|entry| {
                let is_dir = entry.file_type().unwrap().is_dir();
                let path = entry.path();
                let the = path
                    .strip_prefix(cwd.clone())
                    .unwrap()
                    .display();
                println!("{}{}", the, if is_dir { "/" } else { "" });
                is_dir
            })
            .map(|entry| entry.into_path())
            .collect::<Vec<_>>();

        for entry in dirs {
            queue.push_back(entry);
        }
    }
    Ok(())
}

fn entries(cwd: &Path) -> Vec<DirEntry> {
    WalkBuilder::new(cwd)
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
