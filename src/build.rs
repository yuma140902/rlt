use std::process::Command;

use rlt::cli::BuildArgs;

pub fn build(_args: BuildArgs) {
    let mut command = Command::new("uplatex");
    command.args(&[
        "-halt-on-error",
        "-interaction=nonstopmode",
        "-file-line-error",
        "-recorder",
        "main.ltx",
    ]);
    let mut process = command.spawn().expect("Failed to run uplatex");
    process.wait().expect("Failed to wait uplatex");

    let mut command = Command::new("uplatex");
    command.args(&[
        "-halt-on-error",
        "-interaction=nonstopmode",
        "-file-line-error",
        "-recorder",
        "main.ltx",
    ]);
    let mut process = command.spawn().expect("Failed to run uplatex");
    process.wait().expect("Failed to wait uplatex");

    let mut command = Command::new("uplatex");
    command.args(&[
        "-halt-on-error",
        "-interaction=nonstopmode",
        "-file-line-error",
        "-recorder",
        "main.ltx",
    ]);
    let mut process = command.spawn().expect("Failed to run uplatex");
    process.wait().expect("Failed to wait uplatex");

    println!("done");
}
