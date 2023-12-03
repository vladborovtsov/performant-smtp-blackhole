use native_tls::Identity;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite};
use tokio::net::TcpListener as TokioTcpListener;
use tokio_native_tls::{TlsAcceptor as TokioTlsAcceptor, TlsAcceptor};
use std::fs::File;
use std::io::Read;
use std::net::ToSocketAddrs;
use std::process::exit;
use std::sync::Arc;
use futures::future::select_all;
use std::error::Error;
use log::{debug, error, info};



fn parse_listeners(listeners: Option<&str>) -> Result<Vec<(String, u16)>, Box<dyn std::error::Error>> {
    match listeners {
        Some(listeners_str) if !listeners_str.is_empty() => {
            listeners_str.split(',')
                .map(|s| {
                    s.to_socket_addrs()
                        .map_err(|e| e.into())
                        .and_then(|mut addrs|
                            addrs.next()
                                .ok_or_else(|| "Invalid address".into())
                                .map(|addr| (addr.ip().to_string(), addr.port()))
                        )
                })
                .collect::<Result<Vec<_>, _>>()
        },
        _ => Ok(Vec::new())
    }

}

#[allow(unused_variables)] //TODO: remove this ones function body is implemented.
async fn handle_client(stream: impl AsyncRead + AsyncWrite + Unpin, is_smtps: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Your code to handle the client connection goes here.
    // For example, you might read from or write to the stream.

    // If something goes wrong, return an Error using the `?` operator or `Err(e)`.

    Ok(()) // If everything is fine, return Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let smtp_listeners_str = std::env::var("SMTP_LISTENERS").ok();
    let smtps_listeners_str = std::env::var("SMTPS_LISTENERS").ok();

    debug!("SMTP_LISTENERS string:{}", smtp_listeners_str.clone().unwrap_or_default().to_string());
    debug!("SMTPS_LISTENERS string:{}", smtps_listeners_str.clone().unwrap_or_default().to_string());

    let smtp_addrs = parse_listeners(smtp_listeners_str.as_deref())?;
    let smtps_addrs = parse_listeners(smtps_listeners_str.as_deref())?;

    let mut tokio_tls_acceptor: Option<Arc<TlsAcceptor>> = None;
    if !smtps_addrs.is_empty() {
        let cert_path = std::env::var("CERT_PATH").expect("CERT_PATH not set");
        let password = std::env::var("CERT_PASSWORD").expect("CERT_PASSWORD not set");
        let mut file = File::open(cert_path)?;
        let mut identity_bytes = vec![];
        file.read_to_end(&mut identity_bytes)?;
        let identity = Identity::from_pkcs12(&identity_bytes, &password)?;
        let tls_acceptor = native_tls::TlsAcceptor::builder(identity).build()?;
        tokio_tls_acceptor = Option::from(Arc::new(TokioTlsAcceptor::from(tls_acceptor)));
    }

    let mut listeners = Vec::new();


    for addr in smtp_addrs {
        let listener = TokioTcpListener::bind(addr.clone()).await?;
        info!("Listening at {:?}", addr);
        listeners.push((listener, false));  // false for SMTP
    }

    for addr in smtps_addrs {
        let listener = TokioTcpListener::bind(addr).await?;
        listeners.push((listener, true));  // true for SMTPS
    }

    if listeners.is_empty() {
        error!("Error: No SMTP or SMTPS listeners defined.");
        exit(1);
    }

    let mut futures = Vec::new();

    for (listener, is_smtps) in &listeners {
        let maybe_acceptor = tokio_tls_acceptor.clone();
        let fut = Box::pin(async move {
            match listener.accept().await {
                Ok((socket, _)) => {
                    if *is_smtps {
                        if let Some(acceptor) = maybe_acceptor {
                            match acceptor.accept(socket).await {
                                Ok(secure_socket) => {
                                    handle_client(secure_socket, true).await
                                },
                                Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
                            }
                        }
                        else {
                            return Err(Box::try_from("About to handle SMTPS socket, but no identity loaded.").unwrap())
                        }
                    } else {
                        handle_client(socket, false).await
                    }
                },
                Err(e) => Err(Box::new(e) as Box<dyn Error + Send + Sync>),
            }
        });
        futures.push(fut);
    }



    loop {
        if !futures.is_empty() {
            let (result, _index, remaining_futures) = select_all(futures).await;
            // Handle the result of the future here (e.g., log error)
            futures = remaining_futures;
        } else {
            break; // Exit loop if no futures are left
        }
    }

    Ok(())
}
