//extern crate native_tls;

use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

use native_tls::{Protocol, TlsConnector};

pub struct Email {
    email: Message,
}

pub struct Client {
    login: String,
    password: String,
    server: String,
    port: u16,
}

impl Client {
    pub fn new(login: &str, password: &str, server: &str, port: u16) -> Client {
        Client {
            login: login.to_string(),
            password: password.to_string(),
            server: server.to_string(),
            port: port,
        }
    }
}

impl Email {
    pub fn new(from: &str, to: &str, subject: &str, body: &str) -> Email {
        let email = Message::builder()
            .from(from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .body(String::from(body))
            .unwrap();

        Email { email: email }
    }

    pub fn send(&self, client: &Client) {
        let smtp_credentials = Credentials::new(client.login.clone(), client.password.clone());

        let mut mailer = SmtpTransport::relay(&client.server)
            .unwrap()
            .credentials(smtp_credentials)
            .build();

        match mailer.send(&self.email) {
            Ok(_) => (),
            Err(e) => println!("Error {:?}", e),
        }
    }
}
