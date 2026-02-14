#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use itertools::Itertools;
use wks::alias_boundary_left;
use wks::boundary_left_to_right;
use wks::prelude::*;

fn main() -> Result<()> {
    let args = env::args();
    let mut args = args.skip(1);
    let buffer_indentation = args.next().unwrap();
    let line_indentation = args.next().unwrap();
    let surrounding_left: String = args
        .next()
        .and_then(|arg| arg.parse().ok())
        .unwrap();
    let surrounding_left = alias_boundary_left(&surrounding_left);
    let surrounding_right = boundary_left_to_right(surrounding_left);
    let no_indent_innards = [r#"""""#, "```", "'''"];
    let indent_innards = no_indent_innards
        .contains(&surrounding_left)
        .not();
    let mut printed_starter = false;
    for line in io::stdin()
        .lines()
        .map(Result::unwrap)
    {
        let contains_own_indent = line.trim_start() != line;
        if !printed_starter {
            printed_starter = true;
            // if selection contains its own indentation, we should also indent our new wrapping characters
            // if the selection is whitespace-free, we are trying to wrap something in the middle of the line and so should *not*
            // indent the surrounders that will appear inside of a line
            if contains_own_indent {
                print!("{}", line_indentation);
            }
            println!("{}", surrounding_left);
        }
        if !contains_own_indent {
            print!("{}", line_indentation);
        }
        if indent_innards {
            print!("{}", buffer_indentation);
        }
        println!("{}", line);
    }
    print!("{}", line_indentation);
    println!("{}", surrounding_right);
    Ok(())
}
