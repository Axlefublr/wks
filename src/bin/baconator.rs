#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

#[derive(Parser)]
struct Cli {
    action: Action,
    path: PathBuf,
    line: usize,
    column: usize,
}

#[derive(Clone, ValueEnum, PartialEq)]
enum Action {
    First,
    Next,
    Prev,
}

struct DiagInfo {
    path: PathBuf,
    line: usize,
    column: usize,
}

impl TryFrom<&str> for DiagInfo {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut the = value.split(':');
        let path: PathBuf = the
            .next()
            .ok_or(anyhow!("path missing"))?
            .into();
        let line: usize = the
            .next()
            .ok_or(anyhow!("line missing"))?
            .parse()?;
        let column: usize = the
            .next()
            .ok_or(anyhow!("column missing"))?
            .parse()?;
        Ok(Self { path, line, column })
    }
}

impl Display for DiagInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.path.display(), self.line, self.column)
    }
}

#[allow(clippy::skip_while_next)]
fn main() -> Result<()> {
    let Cli {
        action,
        path: editor_path,
        line: editor_line,
        column: editor_column,
    } = Cli::parse();
    let bacon_locations = fs::read_to_string(".bacon-locations").context(".bacon_locations is missing")?;
    let locations: Vec<DiagInfo> = bacon_locations
        .lines()
        .filter_map(|line| line.try_into().ok())
        .collect();
    // don't move if there are no diagnostics to begin with
    if locations.is_empty() {
        return Ok(());
    }
    // if we don't even have the file, or it doesn't parse, or it's an invalid index, next should “arrive” to 0, rather than 1, which would be the *second* diagnostic
    let current_index = fs::read_to_string(".bacon-current")
        .ok()
        .and_then(|the| the.trim().parse::<usize>().ok())
        .filter(|the| the < &locations.len());
    let targeted_index = if let Some(current_index) = current_index {
        match action {
            Action::Next => (current_index + 1).min(locations.len().saturating_sub(1)),
            Action::Prev => current_index.saturating_sub(1),
            Action::First => 0,
        }
    } else {
        0
    };
    if let Some(targeted_location) = locations
        .into_iter()
        .nth(targeted_index)
    {
        println!("{}", targeted_location);
        fs::write(".bacon-current", targeted_index.to_string())?;
    };
    Ok(())
}
