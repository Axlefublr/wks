#![allow(unused_variables)]
#![allow(dead_code)]

use url::Url;

use wks::prelude::*;

#[derive(Debug, Default, PartialEq)]
struct RepoInfo {
    this: String,
    repo: String,
    brath: Option<String>,
    line: Option<usize>,
}

impl RepoInfo {
    pub fn basic<T: Into<String>>(this: T, repo: T) -> Self {
        Self {
            this: this.into(),
            repo: repo.into(),
            ..Default::default()
        }
    }

    pub fn brath<T: Into<String>>(this: T, repo: T, brath: T) -> Self {
        Self {
            this: this.into(),
            repo: repo.into(),
            brath: Some(brath.into()),
            ..Default::default()
        }
    }

    pub fn line<T: Into<String>>(this: T, repo: T, brath: T, line: usize) -> Self {
        Self {
            this: this.into(),
            repo: repo.into(),
            brath: Some(brath.into()),
            line: Some(line),
        }
    }
}

impl TryFrom<&str> for RepoInfo {
    type Error = anyhow::Error;

    fn try_from(provided_url: &str) -> Result<Self, Self::Error> {
        let Ok(mut url) = Url::parse(provided_url) else {
            if let Some(repo) = provided_url.split('/').next_back() {
                return Ok(Self::basic(provided_url, repo));
            } else {
                return Ok(Self::basic(provided_url, provided_url));
            };
        };
        let host = url
            .host_str()
            .ok_or_else(|| anyhow!("no host in url"))?;
        let was_raw = if host == "raw.githubusercontent.com" {
            url.set_host(Some("github.com"))
                .expect("we aren't removing the host");
            true
        } else if host == "github.com" {
            false
        } else {
            bail!("not a github url")
        };
        let line: Option<usize> = url
            .fragment()
            .and_then(|fragment| {
                let mut the = fragment.chars();
                if the.next()? != 'L' {
                    return None;
                }
                let mut buf = String::new();
                the.take_while(|ch| ch.is_ascii_digit())
                    .for_each(|ch| buf.push(ch));
                buf.parse().ok()
            });
        let mut segments = url
            .path_segments()
            .ok_or_else(|| anyhow!("no owner or repo in url"))?;
        let owner = segments
            .next()
            .ok_or_else(|| anyhow!("no owner in url"))?;
        let repo = segments
            .next()
            .ok_or_else(|| anyhow!("no repo in url"))?;
        if was_raw.not() {
            match segments.next() {
                Some("tree") | Some("blob") | Some("blame") | Some("commit") => (),
                _ => {
                    return Ok(Self {
                        this: provided_url.to_owned(),
                        repo: repo.to_owned(),
                        brath: None,
                        line: None,
                    });
                },
            }
        }
        let segments: Vec<_> = segments.collect();
        let brath = segments
            .is_empty()
            .not()
            .then_some(segments.join("/"));
        let this = if was_raw {
            format!("https://github.com/{owner}/{repo}")
        } else {
            provided_url.to_owned()
        };
        Ok(Self {
            this,
            repo: repo.to_owned(),
            brath,
            line,
        })
    }
}

fn main() -> Result<()> {
    let provided_url = env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("url not provided"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shorthand() {
        let ins = "repo";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(out, RepoInfo::basic(ins, "repo"));
    }

    #[test]
    fn shorthand_owner() {
        let ins = "owner/repo";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(out, RepoInfo::basic(ins, "repo"));
    }

    #[test]
    fn normal_nothing() {
        let ins = "https://github.com/owner/repo";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(out, RepoInfo::basic(ins, "repo"));
    }

    #[test]
    fn normal_nothing_trail() {
        let ins = "https://github.com/owner/repo/";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(out, RepoInfo::basic(ins, "repo"));
    }

    #[test]
    fn branch() {
        let ins = "https://github.com/owner/repo/tree/main";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(out, RepoInfo::brath(ins, "repo", "main"));
    }

    #[test]
    fn branch_dir() {
        let ins = "https://github.com/owner/repo/tree/main/src/lib";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(out, RepoInfo::brath(ins, "repo", "main/src/lib"));
    }

    #[test]
    fn branch_file() {
        let ins = "https://github.com/owner/repo/blob/main/src/main.rs";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(out, RepoInfo::brath(ins, "repo", "main/src/main.rs"));
    }

    #[test]
    fn branch_file_line() {
        let ins = "https://github.com/owner/repo/blob/main/src/main.rs#L15";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(out, RepoInfo::line(ins, "repo", "main/src/main.rs", 15));
    }

    #[test]
    fn branch_file_param() {
        let ins = "https://github.com/owner/repo/blob/main/src/main.rs?plain=1";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(out, RepoInfo::brath(ins, "repo", "main/src/main.rs"));
    }

    #[test]
    fn branch_file_param_line() {
        let ins = "https://github.com/owner/repo/blob/main/src/main.rs?plain=1#L10";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(out, RepoInfo::line(ins, "repo", "main/src/main.rs", 10));
    }

    #[test]
    fn blame_file() {
        let ins = "https://github.com/owner/repo/blame/main/src/main.rs";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(out, RepoInfo::brath(ins, "repo", "main/src/main.rs"));
    }

    #[test]
    fn commit() {
        let ins = "https://github.com/owner/repo/tree/0123456789abcdef0123456789abcdef01234567";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(
            out,
            RepoInfo::brath(ins, "repo", "0123456789abcdef0123456789abcdef01234567")
        );
    }

    #[test]
    fn commit_file() {
        let ins = "https://github.com/owner/repo/blob/0123456789abcdef0123456789abcdef01234567/src/main.rs";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(
            out,
            RepoInfo::brath(
                ins,
                "repo",
                "0123456789abcdef0123456789abcdef01234567/src/main.rs"
            )
        );
    }

    #[test]
    fn commit_file_line() {
        let ins =
            "https://github.com/owner/repo/blob/0123456789abcdef0123456789abcdef01234567/src/main.rs#L50";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(
            out,
            RepoInfo::line(
                ins,
                "repo",
                "0123456789abcdef0123456789abcdef01234567/src/main.rs",
                50
            )
        );
    }

    #[test]
    fn commit_file_dir() {
        let ins = "https://github.com/owner/repo/tree/0123456789abcdef0123456789abcdef01234567/src";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(
            out,
            RepoInfo::brath(ins, "repo", "0123456789abcdef0123456789abcdef01234567/src")
        );
    }

    #[test]
    fn commit_view() {
        let ins = "https://github.com/owner/repo/commit/0123456789abcdef0123456789abcdef01234567";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(
            out,
            RepoInfo::brath(ins, "repo", "0123456789abcdef0123456789abcdef01234567")
        );
    }

    #[test]
    fn tag() {
        let ins = "https://github.com/owner/repo/tree/v1.0.0";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(out, RepoInfo::brath(ins, "repo", "v1.0.0"));
    }

    #[test]
    fn tag_file() {
        let ins = "https://github.com/owner/repo/blob/v1.2.3/src/main.rs";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(out, RepoInfo::brath(ins, "repo", "v1.2.3/src/main.rs"));
    }

    #[test]
    fn raw_file() {
        let ins = "https://raw.githubusercontent.com/owner/repo/main/README.md";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(
            out,
            RepoInfo::brath("https://github.com/owner/repo", "repo", "main/README.md")
        );
    }

    #[test]
    fn raw_permanent_file() {
        let ins = "https://raw.githubusercontent.com/owner/repo/0123456789abcdef0123456789abcdef01234567/src/main.rs";
        let out = RepoInfo::try_from(ins).unwrap();
        assert_eq!(
            out,
            RepoInfo::brath(
                "https://github.com/owner/repo",
                "repo",
                "0123456789abcdef0123456789abcdef01234567/src/main.rs"
            )
        );
    }
}
