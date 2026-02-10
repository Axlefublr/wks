#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

const ML_PER_DAY: f32 = 11.4;
const HEIGHT_100_BOTTLE: f32 = 10.5;
const ML_IN_BOTTLE: f32 = 100.;

fn main() -> Result<()> {
    let total_ml = env::args()
        .skip(1)
        .map(|arg| {
            let mut split = arg.split('/');
            let current_cm = split
                .next()
                .unwrap()
                .parse::<f32>()
                .unwrap();
            let max_cm = split
                .next()
                .and_then(|the| the.parse::<f32>().ok())
                .unwrap_or(HEIGHT_100_BOTTLE);
            let max_ml = split
                .next()
                .and_then(|the| the.parse::<f32>().ok())
                .unwrap_or(ML_IN_BOTTLE);
            let ratio = current_cm / max_cm;
            max_ml * ratio
        })
        .sum::<f32>();
    let lasts_days = total_ml / ML_PER_DAY;
    println!("Total: {:.2}ml", total_ml);
    println!("Lasts: {:.2} days", lasts_days);
    Ok(())
}
