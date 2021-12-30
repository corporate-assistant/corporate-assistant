//extern crate native_tls;

use lettre::{
    smtp::authentication::Credentials, smtp::authentication::Mechanism, ClientSecurity,
    ClientTlsParameters, SmtpClient, Transport,
};

use lettre_email::EmailBuilder;

use native_tls::{Protocol, TlsConnector};

pub struct Email {
    email: lettre_email::Email,
}

pub struct EmailServer {
    addr: String,
    port: u16,
}

impl EmailServer {
    pub fn new(server: &str, port: u16) -> EmailServer {
        EmailServer {
            addr: server.to_string(),
            port: port,
        }
    }
}

impl Email {
    pub fn new(from: &str, to: &str, subject: &str, body: &str) -> Email {
        let email = EmailBuilder::new()
            .from(from)
            .to(to)
            .subject(subject)
            .text(body)
            .build()
            .unwrap();

        Email { email: email }
    }

    pub fn send(&self, login: &str, password: &str, server: &EmailServer) {
        let mut tls_builder = TlsConnector::builder();

        tls_builder.min_protocol_version(Some(Protocol::Tlsv12));
        let tls_parameters =
            ClientTlsParameters::new(server.addr.clone(), tls_builder.build().unwrap());

        let smtp_credentials = Credentials::new(login.to_string().clone(), password.to_string().clone());

        let mut mailer = SmtpClient::new(
            (server.addr.clone(), server.port),
            ClientSecurity::Required(tls_parameters),
        )
        .unwrap()
        .authentication_mechanism(Mechanism::Plain)
        .credentials(smtp_credentials)
        .transport();

        match mailer.send(self.email.clone().into()) {
            Ok(_) => (),
            Err(e) => println!("Error {:?}", e),
        }
        mailer.close();
    }
}
