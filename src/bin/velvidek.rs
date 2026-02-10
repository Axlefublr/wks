#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

fn main() -> Result<()> {
    env::args()
        .skip(1)
        .map(|arg| {
            arg.chars()
                .map(|chr| match chr {
                    'f' => '1',
                    'd' => '2',
                    's' => '3',
                    'r' => '4',
                    'e' => '5',
                    'w' => '6',
                    'v' => '7',
                    'c' => '8',
                    'x' => '9',
                    'a' => '0',
                    other => other,
                })
                .collect::<Cow<str>>()
        })
        .for_each(|transformed_number| println!("{transformed_number}"));
    Ok(())
}
