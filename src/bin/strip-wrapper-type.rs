#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

fn main() -> Result<()> {
    let surrounding_left: char = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap();
    let surrounding_left = match surrounding_left {
        'b' => '(',
        'B' => '{',
        't' => '<',
        's' => '[',
        'p' => '|',
        other => other,
    };
    let surrounding_right: char = match surrounding_left {
        '(' => ')',
        '{' => '}',
        '<' => '>',
        '[' => ']',
        other => other,
    };
    let mut the = String::new();
    let stdin = io::stdin()
        .read_to_string(&mut the)
        .expect("stdin");
    let left = the.find(surrounding_left);
    let right = the.rfind(surrounding_right);
    print!(
        "{}",
        &the[{ if let Some(left) = left { left + 1 } else { 0 } }..{
            if let Some(right) = right { right } else { the.len() }
        }]
    );
    Ok(())
}
