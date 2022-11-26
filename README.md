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
- Very basic logging


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

## Examples

#### Getting CA certificate
```
$ curl http://127.0.0.1:3030/api/v1.0/get-ca-cert
{"ca_cert_base64":"LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0tCk1JSUJqekNDQVVFQ0ZDbFBmWWtOVGw2TlZLRVZRVndMeEJSOCtiNG9NQVVHQXl0bGNEQnFNUXN3Q1FZRFZRUUcKRXdKVlV6RU5NQXNHQTFVRUNBd0VWWFJoYURFTk1Bc0dBMVVFQnd3RVRHVm9hVEVZTUJZR0ExVUVDZ3dQVG05MApjbVZoYkdOaExDQkpibU11TVFzd0NRWURWUVFMREFKRFFURVdNQlFHQTFVRUF3d05ibTkwY21WaGJHTmhMbU52CmJUQWVGdzB5TWpFeE1qQXlNek14TVRaYUZ3MHlOREV3TWpBeU16TXhNVFphTUdveEN6QUpCZ05WQkFZVEFsVlQKTVEwd0N3WURWUVFJREFSVmRHRm9NUTB3Q3dZRFZRUUhEQVJNWldocE1SZ3dGZ1lEVlFRS0RBOU9iM1J5WldGcwpZMkVzSUVsdVl5NHhDekFKQmdOVkJBc01Ba05CTVJZd0ZBWURWUVFEREExdWIzUnlaV0ZzWTJFdVkyOXRNQ293CkJRWURLMlZ3QXlFQU5INTF5cVdDcWdGUkhhNXF5UzFHcEdlZm9adlB4Y09LRlRtSE5SQXFhaDR3QlFZREsyVncKQTBFQUtJU1FzN2VBT2dISDRndmo1U1BNMytvZnhNMDRYNjRtbVVudGIydDI3VDArK3lERXFJOVRXRW1XZHU4VwpOMGNOcFZXc3c3LytBd1hwSnlXdGNMZVhEdz09Ci0tLS0tRU5EIENFUlRJRklDQVRFLS0tLS0K","status":"SUCCESS","status_message":"SUCCESS"}
```

#### Getting certificate from CSR
```
curl http://127.0.0.1:3030/api/v1.0/create-from-csr -H "Content-Type: application/json" --request POST --data '{"csr_base64": "LS0tLS1CRUdJTiBDRVJUSUZJQ0FURSBSRVFVRVNULS0tLS0KTUlIbU1JR1pBZ0VBTUdZeEN6QUpCZ05WQkFZVEFsVlRNUTB3Q3dZRFZRUUlEQVJWZEdGb01RMHdDd1lEVlFRSApEQVJNWldocE1SWXdGQVlEVlFRS0RBMU9iM1J5WldGc0xDQkpibU11TVFzd0NRWURWUVFMREFKSlZERVVNQklHCkExVUVBd3dMYm05MGNtVmhiQzVqYjIwd0tqQUZCZ01yWlhBRElRQnczN0FKUFV4a0FpNktHNEpYUGxURlZYSzcKcXhJZURaMjJsWmcyTWNoQTY2QUFNQVVHQXl0bGNBTkJBRDlUTkowZTRSQ295NzFRbWhkc3hSTFhpZHR4K3kxVQpHbFlXU0NyeGlhSm1ZcHVlZ2o5NTRCRE1MNXlqWWdaRVpIT1dKQS9tS2x5eWQ4UGRpckIzd3dzPQotLS0tLUVORCBDRVJUSUZJQ0FURSBSRVFVRVNULS0tLS0K"}'
{"signed_cert_base64":"LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0tCk1JSUJtekNDQVUwQ0FRRXdCUVlESzJWd01Hb3hDekFKQmdOVkJBWVRBbFZUTVEwd0N3WURWUVFJREFSVmRHRm8KTVEwd0N3WURWUVFIREFSTVpXaHBNUmd3RmdZRFZRUUtEQTlPYjNSeVpXRnNZMkVzSUVsdVl5NHhDekFKQmdOVgpCQXNNQWtOQk1SWXdGQVlEVlFRRERBMXViM1J5WldGc1kyRXVZMjl0TUI0WERUSXlNVEV5TmpFM01UVXpNRm9YCkRUSXpNVEV5TmpFM01UVXpNRm93WmpFTE1Ba0dBMVVFQmhNQ1ZWTXhEVEFMQmdOVkJBZ01CRlYwWVdneERUQUwKQmdOVkJBY01CRXhsYUdreEZqQVVCZ05WQkFvTURVNXZkSEpsWVd3c0lFbHVZeTR4Q3pBSkJnTlZCQXNNQWtsVQpNUlF3RWdZRFZRUUREQXR1YjNSeVpXRnNMbU52YlRBcU1BVUdBeXRsY0FNaEFIRGZzQWs5VEdRQ0xvb2JnbGMrClZNVlZjcnVyRWg0Tm5iYVZtRFl4eUVEcm95RXdIekFkQmdOVkhRNEVGZ1FVMmptajdsNXJTdzB5VmIvdmxXQVkKa0svWUJ3a3dCUVlESzJWd0EwRUFHZ2Y0clRqcTNZVnV0NStvamtQenBLSGE0MmRWZDBvS3RkQTlTQzdYUjVZZgpqU2lST3pYYzdwZ2ltaGNoR3NXT05FSkxZR3VEMTQ2ZUx6T2FGRXlwRHc9PQotLS0tLUVORCBDRVJUSUZJQ0FURS0tLS0tCg==","status":"SUCCESS","status_message":"SUCCESS"}
```