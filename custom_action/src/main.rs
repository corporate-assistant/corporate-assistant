use std::process::Command;

fn main() {
    println!("Hello, world!");


let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
} else {

    let mut dir = std::env::current_exe().unwrap();
    dir.pop();
    dir.pop();
    dir.pop();
    dir.push("custom_action");
    dir.push("scripts");
    dir.push("custom_script.sh");
    println!("Binary path: {}", dir.display());
    Command::new("bash")
            .arg(dir.into_os_string())
            .output()
            .expect("failed to execute process")
};

let hello = output.stdout;

println!("out: {}", String::from_utf8(hello).unwrap());

}
