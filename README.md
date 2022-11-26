# rust-toy-ca
This is a simple service made using [Warp]() and OpenSSL: it will take a CSR and return a certificate signed by the CA.

## DISCLAIMER
Please note that I do not know enoguh about of Rust or OpenSSL. I made this to try to learn more, so don´t be using this in production... and wherever you use this, please be aware that I take no responsability for the outcome.

## Limitations / (bad) Design choices
The following are current known limitations; these might change in the future, but there is no commitment to do so:
- Serial is always `1`
- Only ED25519/EdDSA
- No V3 extensions (aside from `subject_key_identifier1`)
- Input/Output of certs/csr in base64
- No TLS
- No authentication
- No tests
- Naive design
- Certificate valid fixed for 1 year (fixed)


## Cool reading
- https://blog.pinterjann.is/ed25519-certificates.html
- https://github.com/seanmonstar/warp/tree/master/examples
- https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/
- https://learn.microsoft.com/en-us/azure/iot-hub/tutorial-x509-openssl
- https://github.com/Azure/azure-iot-sdk-c/blob/main/tools/CACertificates/certGen.sh
- https://www.digicert.com/kb/ssl-support/openssl-quick-reference-guide.htm



## Running
You need to pass the CA certificate and private key as arguments. Examples:

```
$ cargo run -- --ca-cert-file test_helpers/ca.crt --ca-pkey-file test_helpers/ca.key
```

```
$ ./rust-toy-ca --ca-cert-file test_helpers/ca.crt --ca-pkey-file test_helpers/ca.key
```

