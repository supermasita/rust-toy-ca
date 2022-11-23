use warp::Filter;

use base64::decode;
use openssl::asn1::{Asn1Integer, Asn1Time};
use openssl::bn::BigNum;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::x509::extension::SubjectKeyIdentifier;
use openssl::x509::{X509Builder, X509Req, X509};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use clap::Parser;

#[derive(Debug, Deserialize, Serialize)]
enum Status{
    SUCCESS,
    FAILURE,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// CA pub cert
   #[arg(long)]
   ca_cert_file: String,

   /// CA priv key
   #[arg(long)]
   ca_pkey_file: String,
}

#[derive(Deserialize, Serialize)]
struct SignReq {
    csr_base64: String,
}

#[derive(Deserialize, Serialize)]
struct SignRep {
    signed_cert_base64: String,
    status: Status
}

#[derive(Deserialize, Serialize)]
struct CaCertRep {
    ca_cert_base64: String,
}

fn _load_ca_cert(path: &String) -> String {
    let file_contents: String = fs::read_to_string(path).expect("Couldn´t read the CA certificate file");
    file_contents
}

fn _load_ca_pk(path: &String) -> String {
    let file_contents: String = fs::read_to_string(path).expect("Couldn´t read the CA private key file");
    file_contents
}

fn _encode_it(data: &[u8]) -> String {
    return base64::encode(data);
}

fn _sign_csr_cert(csr_base64: &String, ca_cert: &X509, ca_pkey: &PKey<Private>) -> String {
    let csr_as_vec = decode(csr_base64).unwrap();  // what if broken base64?
    let csr = X509Req::from_pem(&csr_as_vec).unwrap();  // what if broken cert?
    println!("CSR CN {:#?}", &csr.subject_name());

    let one_year = Asn1Time::days_from_now(365).unwrap();
    let now = Asn1Time::days_from_now(0).unwrap();

    let mut cert_builder = X509Builder::new().unwrap();
    let subject_key_identifier = SubjectKeyIdentifier::new()
        .build(&cert_builder.x509v3_context(None, None))
        .unwrap();
    cert_builder
        .append_extension(subject_key_identifier)
        .unwrap();
    cert_builder.set_serial_number(&Asn1Integer::from_bn(&BigNum::from_u32(1).unwrap()).unwrap());
    cert_builder.set_not_before(&now);
    cert_builder.set_not_after(&one_year);
    cert_builder.set_issuer_name(&ca_cert.issuer_name());
    cert_builder.set_subject_name(&csr.subject_name());
    cert_builder.set_pubkey(&csr.public_key().unwrap());
    cert_builder.set_version(csr.version().clone());

    // > The openssl docs mention that ed25519 did not support a digest, so openssl requires 
    // > to pass a NULL ptr as digest algorithm argument. Ref: 
    // > https://github.com/openssl/openssl/blob/master/doc/man1/openssl-ca.pod.in#L204
    // (source: https://github.com/sfackler/rust-openssl/issues/1197 )
    unsafe {
        cert_builder.sign(&ca_pkey, MessageDigest::from_ptr(std::ptr::null()));
    };
    let cert_final = cert_builder.build();
    return _encode_it(&cert_final.to_pem().unwrap());
}

async fn _create_from_csr(
    item: SignReq,
    ca_cert: X509,
    ca_pkey: PKey<Private>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let cert_rep = SignRep {
        signed_cert_base64: _sign_csr_cert(&item.csr_base64, &ca_cert, &ca_pkey),
        status: Status::SUCCESS
    };
    Ok(warp::reply::json(&cert_rep))
}

async fn _get_ca_cert(
    ca_cert: X509,
) -> Result<impl warp::Reply, warp::Rejection> {
    let ca_cert_rep = CaCertRep {
        ca_cert_base64: _encode_it(&ca_cert.to_pem().unwrap()),
    };
    Ok(warp::reply::json(&ca_cert_rep))
}

#[tokio::main]
async fn main() {

    let args = Args::parse();
    let ca_cert = X509::from_pem(_load_ca_cert(&args.ca_cert_file).as_bytes()).unwrap();
    let ca_cert_w = warp::any().map(move || ca_cert.clone());
    let ca_pkey = PKey::private_key_from_pem(_load_ca_pk(&args.ca_pkey_file).as_bytes()).unwrap();
    let ca_pkey_w = warp::any().map(move || ca_pkey.clone());

    let route_get_ca_cert = warp::get()
        .and(warp::path("api"))
        .and(warp::path("v1"))
        .and(warp::path("get-ca-cert"))
        .and(warp::path::end())
        .and(ca_cert_w.clone())
        .and_then(_get_ca_cert);


    let route_create_from_csr = warp::post()
        .and(warp::path("api"))
        .and(warp::path("v1"))
        .and(warp::path("create-from-csr"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(ca_cert_w.clone())
        .and(ca_pkey_w.clone())
        .and_then(_create_from_csr);

    let routes = warp::any().and(route_get_ca_cert.or(route_create_from_csr));

    println!("Running...");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
