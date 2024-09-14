
use std::process::Command;

fn main() {
    let status = Command::new("make")
        .arg("-C")
        .arg("lib/libcss/")
        .status()
        .expect("Failed to execute GNU Make")
        ;

    if !status.success() {
        panic!("Make failed with status: {:?}", status);
    }
}
