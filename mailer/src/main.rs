pub use mailer::{Client, Email};
use std::env;

fn parse_cmdline() -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<u16>,
    Option<String>,
    Option<String>,
) {
    let mut from: Option<String> = None;
    let mut to: Option<String> = None;
    let mut server: Option<String> = None;
    let mut port: Option<u16> = None;
    let mut login: Option<String> = None;
    let mut password: Option<String> = None;

    for arg in env::args() {
        if arg.starts_with("--") {
            let trimmed = arg.trim_start_matches("--");
            let split: Vec<_> = trimmed.split("=").collect();
            let key = split[0];

            match key {
                "from" => from = Some(String::from(split[1])),
                "to" => to = Some(String::from(split[1])),
                "server" => server = Some(String::from(split[1])),
                "port" => port = Some(String::from(split[1]).parse().expect("Not a number")),
                "login" => login = Some(String::from(split[1])),
                "password" => password = Some(String::from(split[1])),
                _ => (),
            }
        }
    }

    (from, to, server, port, login, password)
}

fn main() {
    let (from, to, server, port, login, password) = parse_cmdline();
    let client = Client::new(
        &login.expect("Incorrect login"),
        &password.expect("Incorrect password"),
        &server.expect("Incorrect server"),
        port.expect("Incorrect port"),
    );
    let email = Email::new(
        &from.expect("Incorrect source email address"),
        &to.expect("Incorrect destination email address"),
        "Test Rusta",
        "Test",
    );

    email.send(&client);
}
