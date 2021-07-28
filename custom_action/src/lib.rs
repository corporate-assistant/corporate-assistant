// 1. Make a window where script can be put and by default it
//   we put this script there
// 2. Upon finishing to edit Save a script

use std::path::PathBuf;
use std::process::Command;

pub fn action_creator() {
    // GUI based text editor
    todo!();
}

pub fn action_executor(script_name: PathBuf) {
    // Executing an action from script
    println!("Binary path: {}", script_name.display());
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("bash")
            .arg(script_name.into_os_string())
            .output()
            .expect("failed to execute process")
    };
    let stdout = output.stdout;
    println!("out: {}", String::from_utf8(stdout).unwrap());
}
