#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use itertools::Itertools;
use wks::alias_boundary_left;
use wks::boundary_left_to_right;
use wks::prelude::*;

fn main() -> Result<()> {
    let surrounding_left: String = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap();
    let surrounding_left = alias_boundary_left(&surrounding_left);
    let surrounding_right = boundary_left_to_right(surrounding_left);
    let mut the = String::new();
    io::stdin()
        .read_to_string(&mut the)
        .expect("stdin");
    let left = the.find(surrounding_left);
    let right = the.rfind(surrounding_right);
    print!("{}", {
        let left = if let Some(left) = left {
            let offset = surrounding_left.len();
            let whitespace = &the[(left + offset)..]
                .chars()
                .take_while(|chr| chr.is_indentation())
                .count();
            let newlines = &the[(left + offset)..]
                .chars()
                .skip_while(|chr| chr.is_indentation())
                .take_while(|chr| *chr == '\n')
                .count();
            left + offset + whitespace + newlines
        } else {
            0
        };
        let right = if let Some(right) = right {
            let offset = surrounding_right.len();
            let whitespace = &the[..(right - offset)]
                .chars()
                .rev()
                .take_while(|chr| chr.is_indentation())
                .count();
            let newlines = &the[..(right - offset)]
                .chars()
                .rev()
                .skip_while(|chr| chr.is_indentation())
                .take_while(|chr| *chr == '\n')
                .count();
            right - offset - whitespace - newlines + 1
        } else {
            the.len()
        };
        &the[left..right]
    });
    Ok(())
}
