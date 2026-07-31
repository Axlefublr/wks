#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

fn main() -> Result<()> {
    let brath = env::args()
        .nth(1)
        .expect("provide brath");
    let branchlikes = refs_into_branchlikes(&collect_refs()?);
    let potentials = brath_into_potentials(&brath);
    let resolved = resolve_ref(&branchlikes, &potentials).unwrap_or_else(|| {
        // if no possible branch matches any of the existing branches, it's for sure a commit. hopefully
        // the last element in potentials is always the 1-width branch, no need to resplit
        let likely_commit = potentials.last().unwrap();
        if verify_commit(likely_commit) {
            likely_commit.to_owned()
        } else {
            panic!("this ain't even a commit, what is this shit");
        }
    });
    println!("{}", resolved);
    Ok(())
}

fn collect_refs() -> Result<String> {
    Ok(cmd!("git", "for-each-ref").read()?)
}

fn refs_into_branchlikes(refile: &str) -> Vec<String> {
    refile
        .lines()
        .map(|line| line.trim_ascii())
        .filter(|line| line.is_empty().not())
        .filter(|line| {
            [
                "refs/remotes/origin/HEAD",
                "refs/remotes/upstream/HEAD",
                "refs/stash",
            ]
            .contains(line)
            .not()
        })
        .map(|line| {
            line.trim_start_matches("refs/remotes/upstream/")
                .trim_start_matches("refs/remotes/origin/")
                .trim_start_matches("refs/tags/")
                .trim_start_matches("refs/heads/")
        })
        .map(ToOwned::to_owned)
        .inspect(|the| eprintln!("`{}`", the))
        .collect()
}

fn brath_into_potentials(brath: &str) -> Vec<String> {
    if brath.contains('/').not() {
        // we need at least one slash to include a path, as the branchlike will always be included. this means that we necessarily only have the branch specified, and it's `brath`
        return vec![brath.to_owned()];
    }
    let mut potential_men = Vec::new();
    let pieces = brath
        .split('/')
        .collect::<Vec<_>>();
    for i in (0..(pieces.len())).rev() {
        let the = pieces[0..i].join("/");
        if the.is_empty().not() {
            potential_men.push(the);
        }
    }
    potential_men
}

fn resolve_ref(branchlikes: &Vec<String>, potentials: &Vec<String>) -> Option<String> {
    for potential in potentials {
        for branchlike in branchlikes {
            if potential == branchlike {
                return Some(potential.to_owned());
            }
        }
    }
    None
}

fn verify_commit(likely_commit: &str) -> bool {
    cmd!(
        "git",
        "rev-parse",
        "--verify",
        "--quiet",
        &format!("{likely_commit}^{{commit}}")
    )
    .run()
    .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_branches() {
        let ins = r#"
            refs/heads/10576/anchors/pantos9000
            refs/heads/11700/search_index/useche
        "#;
        let out = refs_into_branchlikes(ins);
        assert_eq!(
            out,
            vec![
                String::from("10576/anchors/pantos9000"),
                String::from("11700/search_index/useche")
            ]
        );
    }

    #[test]
    fn origin_branches() {
        let ins = r#"
            refs/remotes/origin/Axlefublr/buffer_nth
            refs/remotes/origin/Axlefublr/enable_diagnostics
        "#;
        let out = refs_into_branchlikes(ins);
        assert_eq!(
            out,
            vec![
                String::from("Axlefublr/buffer_nth"),
                String::from("Axlefublr/enable_diagnostics")
            ]
        );
    }

    #[test]
    fn upstream_branches() {
        let ins = r#"
            refs/remotes/upstream/config_refactor_v2
            refs/remotes/upstream/dependabot/cargo/rust-dependencies-c55b2a2244
        "#;
        let out = refs_into_branchlikes(ins);
        assert_eq!(
            out,
            vec![
                String::from("config_refactor_v2"),
                String::from("dependabot/cargo/rust-dependencies-c55b2a2244")
            ]
        );
    }

    #[test]
    fn tags() {
        let ins = r#"
            refs/tags/25.07.1
            refs/tags/v0.0.10
        "#;
        let out = refs_into_branchlikes(ins);
        assert_eq!(out, vec![String::from("25.07.1"), String::from("v0.0.10")]);
    }

    #[test]
    fn blocked() {
        let ins = r#"
            refs/remotes/origin/HEAD
            refs/remotes/upstream/HEAD
            refs/stash
        "#;
        let out = refs_into_branchlikes(ins);
        assert_eq!(out, Vec::<String>::new());
    }

    #[test]
    fn one_of_each() {
        let ins = r#"
            refs/heads/10576/anchors/pantos9000
            refs/heads/dev
            refs/remotes/origin/Axlefublr/buffer_nth
            refs/remotes/origin/HEAD
            refs/remotes/upstream/25.07.x
            refs/remotes/upstream/HEAD
            refs/stash
            refs/tags/22.03
        "#;
        let out = refs_into_branchlikes(ins);
        assert_eq!(
            out,
            vec![
                String::from("10576/anchors/pantos9000"),
                String::from("dev"),
                String::from("Axlefublr/buffer_nth"),
                String::from("25.07.x"),
                String::from("22.03"),
            ]
        );
    }

    #[test]
    fn potentialing() {
        let ins = "guarantees/orgasm/albany/permit/routine.rs";
        let out = brath_into_potentials(ins);
        assert_eq!(
            out,
            vec![
                String::from("guarantees/orgasm/albany/permit"),
                String::from("guarantees/orgasm/albany"),
                String::from("guarantees/orgasm"),
                String::from("guarantees"),
            ]
        );
    }

    #[test]
    fn potentialing_uselessly() {
        let ins = "guarantees";
        let out = brath_into_potentials(ins);
        assert_eq!(out, vec![String::from("guarantees"),]);
    }

    #[test]
    fn resolvation() {
        let potentials = vec![
            String::from("guarantees/orgasm/albany/permit"),
            String::from("guarantees/orgasm/albany"),
            String::from("guarantees/orgasm"),
            String::from("guarantees"),
        ];
        let branchlikes = vec![
            String::from("guarantees"),
            String::from("guarantees/orgasm"),
            String::from("guarantees/orgasm/albany"),
            String::from("guarantees/orgasm/albany/permit"),
        ];
        let out = resolve_ref(&branchlikes, &potentials);
        assert_eq!(out, Some("guarantees/orgasm/albany/permit".to_owned()));
    }

    #[test]
    fn resolvation_rev() {
        let potentials = vec![
            String::from("guarantees/orgasm/albany/permit"),
            String::from("guarantees/orgasm/albany"),
            String::from("guarantees/orgasm"),
            String::from("guarantees"),
        ];
        let branchlikes = vec![
            String::from("guarantees/orgasm/albany/permit"),
            String::from("guarantees/orgasm/albany"),
            String::from("guarantees/orgasm"),
            String::from("guarantees"),
        ];
        let out = resolve_ref(&branchlikes, &potentials);
        assert_eq!(out, Some("guarantees/orgasm/albany/permit".to_owned()));
    }
}
