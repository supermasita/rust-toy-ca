# rust-toy-ca


## Limitations
- Serial is always `1`
- Only ED25519/EdDSA
- No V3 extensions (aside from `subject_key_identifier1`)
- Input/Output of certs/csr in base64
- no tls
- no auth
- no tests
- naive/optimistic design
- cert duration fixed to 1 year


## Cool reading

https://blog.pinterjann.is/ed25519-certificates.html
https://github.com/seanmonstar/warp/tree/master/examples
https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/
https://learn.microsoft.com/en-us/azure/iot-hub/tutorial-x509-openssl
https://github.com/Azure/azure-iot-sdk-c/blob/main/tools/CACertificates/certGen.sh
https://www.digicert.com/kb/ssl-support/openssl-quick-reference-guide.htm



## notes

cargo run -- --ca-cert-file test_helpers/ca.crt --ca-pkey-file test_helpers/ca.key 