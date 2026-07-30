#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

fn main() -> Result<()> {
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
        todo
        "#;
        let out = refs_into_branchlikes(ins);
        assert_eq!(out, Vec::<String>::new());
    }
}
