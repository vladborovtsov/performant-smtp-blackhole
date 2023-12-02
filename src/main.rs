use tokio::net::TcpListener;
use native_tls::Identity;
use tokio_native_tls::TlsAcceptor as TokioTlsAcceptor;
use std::fs::File;
use std::io::Read;
use tokio::io::{AsyncRead, AsyncWrite};


async fn handle_client<S>(mut stream: S, is_smtps: bool)
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    // TODO: Implement SMTP protocol handling here.
    // If STARTTLS command is received and is_smtps is false, upgrade to TLS.
    stream;
    is_smtps;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert_path = std::env::var("CERT_PATH").expect("CERT_PATH not set");
    let mut file = File::open(cert_path)?;
    let password = std::env::var("CERT_PASSWORD").expect("CERT_PASSWORD not set");
    let mut identity_bytes = vec![];
    file.read_to_end(&mut identity_bytes)?;
    let identity = Identity::from_pkcs12(&identity_bytes, &password)?;

    let listener_standard = TcpListener::bind("127.0.0.1:25").await?;
    let listener_standard_alt = TcpListener::bind("127.0.0.1:2525").await?;
    let listener_default = TcpListener::bind("127.0.0.1:587").await?;
    let listener_smtps = TcpListener::bind("127.0.0.1:465").await?;

    let tls_acceptor = native_tls::TlsAcceptor::builder(identity).build()?;
    let tokio_tls_acceptor = TokioTlsAcceptor::from(tls_acceptor);


    loop {
        tokio::select! {
            Ok((socket, _)) = listener_standard.accept() => {
                tokio::spawn(handle_client(socket, false));
            },
            Ok((socket, _)) = listener_standard_alt.accept() => {
                tokio::spawn(handle_client(socket, false));
            },
            Ok((socket, _)) = listener_default.accept() => {
                tokio::spawn(handle_client(socket, false));
            },
            Ok((socket, _)) = listener_smtps.accept() => {
                let acceptor = tokio_tls_acceptor.clone();
                let secure_socket = acceptor.accept(socket).await?;
                tokio::spawn(handle_client(secure_socket, true));
            }
        }
    }
}
