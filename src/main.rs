use base64::decode;
use clap::Parser;
use log::{debug, error, info, trace, warn};
use log4rs;
use openssl::asn1::{Asn1Integer, Asn1Time};
use openssl::bn::BigNum;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::x509::extension::SubjectKeyIdentifier;
use openssl::x509::{X509Builder, X509Req, X509};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use warp::Filter;
mod logging;
use std::net::SocketAddr;

/// Request status (CA cert request, CSR signing)
#[derive(Debug, Deserialize, Serialize)]
enum Status {
    SUCCESS,
    FAILURE,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// CLI arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// CA public certificate path on storage
    #[arg(long)]
    ca_cert_file: String,

    /// CA private key path on storage
    #[arg(long)]
    ca_pkey_file: String,

    /// IP:Port for the server to listen
    #[arg(long)]
    listen: String,
}

/// Signing request
#[derive(Deserialize, Serialize)]
struct SignReq {
    /// CSR base64 encoded
    csr_base64: String,
}

/// Signing request reply
#[derive(Deserialize, Serialize)]
struct SignRep {
    /// the signed certificate as a PEM, base64 encoded
    signed_cert_base64: String,
    /// status
    status: Status,
    /// status message, which can contain the exception message, if there was one
    status_message: String,
}

/// CA certificate reply
#[derive(Deserialize, Serialize)]
struct CaCertRep {
    /// CA public certificate as PEM, base64 encoded
    ca_cert_base64: String,
    /// status
    status: Status,
    /// status message, which can contain the exception message, if there was one
    status_message: String,
}

/// Loads the CA public certificate (PEM) from storage
///
/// # Arguments
/// * `path`: path for certificate on storage
fn load_ca_cert(path: &String) -> String {
    let file_contents: String =
        fs::read_to_string(path).expect("Couldn´t read the CA certificate file");
    file_contents
}

/// Loads the CA private key (PKCS8) from storage
///
/// # Arguments
/// * `path`: path for private key on storage
fn load_ca_pk(path: &String) -> String {
    let file_contents: String =
        fs::read_to_string(path).expect("Couldn´t read the CA private key file");
    file_contents
}

/// Returns base64 encode (String) from passed data
///
/// # Arguments
/// * `data`:
fn to_base64(data: &[u8]) -> String {
    return base64::encode(data);
}

/// Returns a public certificate (PEM) created using the passed CSR
///
/// # Arguments
/// * `csr_base64`: base64 representation of CSR
/// * `ca_cert`
/// * `ca_pkey`
///
/// # Returns
/// * PEM certificate (String)
/// * Status (Status)
/// * Status Message (String)
fn create_cert_from_csr(
    csr_base64: &String,
    ca_cert: &X509,
    ca_pkey: &PKey<Private>,
) -> (String, Status, String) {
    // Decode CSR
    let csr_as_vec = match decode(csr_base64) {
        Ok(c) => c,
        Err(e) => return ("".to_string(), Status::FAILURE, e.to_string()),
    };
    // Extract CSR from PEM
    let csr = match X509Req::from_pem(&csr_as_vec) {
        Ok(c) => c,
        Err(e) => return ("".to_string(), Status::FAILURE, e.to_string()),
    };

    // info!("CSR CN {:#?}", &csr.subject_name());

    // Basic builder stuff
    let mut cert_builder = X509Builder::new().unwrap();
    let subject_key_identifier = SubjectKeyIdentifier::new()
        .build(&cert_builder.x509v3_context(None, None))
        .unwrap();
    cert_builder
        .append_extension(subject_key_identifier)
        .unwrap();
    cert_builder.set_serial_number(&Asn1Integer::from_bn(&BigNum::from_u32(1).unwrap()).unwrap());
    cert_builder.set_issuer_name(&ca_cert.issuer_name());
    cert_builder.set_subject_name(&csr.subject_name());
    cert_builder.set_pubkey(&csr.public_key().unwrap());
    cert_builder.set_version(csr.version().clone());

    // Add expiry dates
    let one_year = Asn1Time::days_from_now(365).unwrap();
    let now = Asn1Time::days_from_now(0).unwrap();
    cert_builder.set_not_before(&now);
    cert_builder.set_not_after(&one_year);

    // Sign certificate
    // NOTE:
    //  > The openssl docs mention that ed25519 did not support a digest, so openssl requires
    //  > to pass a NULL ptr as digest algorithm argument. Ref:
    //  > https://github.com/openssl/openssl/blob/master/doc/man1/openssl-ca.pod.in#L204
    //  (source: https://github.com/sfackler/rust-openssl/issues/1197 )
    unsafe {
        cert_builder.sign(&ca_pkey, MessageDigest::from_ptr(std::ptr::null()));
    };
    // Build certificate
    let cert_final = cert_builder.build();

    return (
        to_base64(&cert_final.to_pem().unwrap()),
        Status::SUCCESS,
        Status::SUCCESS.to_string(),
    );
}

/// Function for `create-from-csr` Warp route
///
/// # Arguments
/// * `item`
/// * `ca_cert`
/// * `ca_pkey`
///
/// # Returns
/// * `SignRep`
async fn create_from_csr(
    item: SignReq,
    ca_cert: X509,
    ca_pkey: PKey<Private>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let cert_tuple = create_cert_from_csr(&item.csr_base64, &ca_cert, &ca_pkey);
    let cert_rep = SignRep {
        signed_cert_base64: cert_tuple.0,
        status: cert_tuple.1,
        status_message: cert_tuple.2,
    };
    Ok(warp::reply::json(&cert_rep))
}

/// Function for `get-ca-cert` Warp route
///
/// # Arguments
/// * `ca_cert`
///
/// # Returns
/// * `CaCertRep`
async fn get_ca_cert(ca_cert: X509) -> Result<impl warp::Reply, warp::Rejection> {
    let ca_cert_rep = CaCertRep {
        ca_cert_base64: to_base64(&ca_cert.to_pem().unwrap()),
        status: Status::SUCCESS,
        status_message: Status::SUCCESS.to_string(),
    };
    Ok(warp::reply::json(&ca_cert_rep))
}

/// rust-toy-ca uses `warp` and `openssl` to sign certificates from CSR
///
/// To run from CLI, you will need to pass the following arguments:
/// * `--ca-cert-file`: path for PEM certificate
/// * `--ca-pkey-file`: path for PKCS8 key
///
/// # Examples
/// ```
/// $ cargo run -- --ca-cert-file test_helpers/ca.crt --ca-pkey-file test_helpers/ca.key
/// ```
///
/// ```
/// $ ./rust-toy-ca --ca-cert-file test_helpers/ca.crt --ca-pkey-file test_helpers/ca.key
/// ```
///
#[tokio::main]
async fn main() {
    // Load logging configuration
    log4rs::init_config(logging::get_log_config()).unwrap();

    // Load CLI args
    let args = Args::parse();

    // Listen address:port
    let listen_addr: SocketAddr = args
        .listen
        .parse()
        .expect("Could not parse passed IP:Port !!!");

    // Load certificates and keys
    // Make a clone for warp
    let ca_cert = X509::from_pem(load_ca_cert(&args.ca_cert_file).as_bytes()).unwrap();
    let ca_cert_w = warp::any().map(move || ca_cert.clone());
    let ca_pkey = PKey::private_key_from_pem(load_ca_pk(&args.ca_pkey_file).as_bytes()).unwrap();
    let ca_pkey_w = warp::any().map(move || ca_pkey.clone());

    // Define routes
    let route_get_ca_cert = warp::get()
        .and(warp::path("api"))
        .and(warp::path("v1.0"))
        .and(warp::path("get-ca-cert"))
        .and(warp::path::end())
        .and(ca_cert_w.clone())
        .and_then(get_ca_cert);
    let route_create_from_csr = warp::post()
        .and(warp::path("api"))
        .and(warp::path("v1.0"))
        .and(warp::path("create-from-csr"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(ca_cert_w.clone())
        .and(ca_pkey_w.clone())
        .and_then(create_from_csr);
    let routes = warp::any()
        .and(route_get_ca_cert.or(route_create_from_csr))
        .with(warp::log("stdout"));

    // Start server
    info!("rust-toy-ca running...");
    warp::serve(routes).run(listen_addr).await;
}
