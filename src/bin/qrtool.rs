#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::os::unix::process::CommandExt;
use std::process::Command;
use wks::prelude::*;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let mut cmd = Command::new("/usr/bin/qrtool");

    fn etc(mut cmd: Command) -> ! {
        panic!("{}", cmd.exec());
    }

    let Some(subcommand) = args.next() else {
        etc(cmd);
    };

    match &subcommand[..] {
        "c" => {
            cmd.args(["encode", "-m", "1", "--optimize-png", "4"]);
            cmd.args(args);
            etc(cmd);
        },
        "t" => {
            cmd.args(["encode", "-m", "1", "-t", "terminal"]);
            cmd.args(args);
            etc(cmd);
        },
        other => {
            cmd.arg(other);
        },
    }
    cmd.args(args);
    etc(cmd);
}
