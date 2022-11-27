# rust-toy-ca
A toy service made using [Warp](https://github.com/seanmonstar/warp) and OpenSSL: it will take a CSR as input and return a certificate signed by the Certificate Authority (CA) credentials you provide.

## DISCLAIMER
Please note that I do not know enoguh about of Rust or OpenSSL. I made this to try to learn more, so don´t be using this in production... and wherever you use this, please be aware that I take no responsability for the outcome.

## Limitations / (bad) Design choices
Needless to say that this package will not provide a complete Certificate Authority. The following are current known limitations; these might change in the future, but I am making no commitment to do so:
- Serial is always `1` (in a real CA, this would be unique per certificate)
- Only ED25519/EdDSA
- No V3 extensions (aside from `subject_key_identifier1`)
- Input/Output of CSR and certificates is base64 encoded (to work around multiline in JSON)
- No TLS/SSL support for the server
- No authentication
- No unit tests
- Naive design (including multiple unhandled Results)
- Certificate valid fixed for 1 year (fixed)
- Very basic logging


## Running
You need to pass the CA certificate and private key as arguments. Example:

```
$ ./rust-toy-ca --ca-cert-file test_helpers/ca.crt --ca-pkey-file test_helpers/ca.key --listen 127.0.0.1:3030
```

## Running with Docker
_Note that you can find the latest Docker image at [Docker Hub](https://hub.docker.com/repository/docker/supermasita/rust-toy-ca)_.

Docker command example, assuming the following:
- your certificate and key (`ca.crt`/`ca.key`) are stored in `/home/user/certs/`
- you want to expose `8888` for any IPv4 in your host
- you want to delete the container once it is stopped

```
$ sudo docker run -v /home/user/certs/:/certs/ -p 8888:8888 --rm supermasita/rust-toy-ca:latest \ 
/usr/bin/rust-toy-ca --ca-cert-file /certs/ca.crt --ca-pkey-file /certs/ca.key --listen 0.0.0.0:8888
```


## Examples

#### Getting CA certificate
```
$ curl http://127.0.0.1:8888/api/v1.0/get-ca-cert
{"ca_cert_base64":"LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0tCk1JSUJqekNDQVVFQ0ZDbFBmWWtOVGw2TlZLRVZRVndMeEJSOCtiNG9NQVVHQXl0bGNEQnFNUXN3Q1FZRFZRUUcKRXdKVlV6RU5NQXNHQTFVRUNBd0VWWFJoYURFTk1Bc0dBMVVFQnd3RVRHVm9hVEVZTUJZR0ExVUVDZ3dQVG05MApjbVZoYkdOaExDQkpibU11TVFzd0NRWURWUVFMREFKRFFURVdNQlFHQTFVRUF3d05ibTkwY21WaGJHTmhMbU52CmJUQWVGdzB5TWpFeE1qQXlNek14TVRaYUZ3MHlOREV3TWpBeU16TXhNVFphTUdveEN6QUpCZ05WQkFZVEFsVlQKTVEwd0N3WURWUVFJREFSVmRHRm9NUTB3Q3dZRFZRUUhEQVJNWldocE1SZ3dGZ1lEVlFRS0RBOU9iM1J5WldGcwpZMkVzSUVsdVl5NHhDekFKQmdOVkJBc01Ba05CTVJZd0ZBWURWUVFEREExdWIzUnlaV0ZzWTJFdVkyOXRNQ293CkJRWURLMlZ3QXlFQU5INTF5cVdDcWdGUkhhNXF5UzFHcEdlZm9adlB4Y09LRlRtSE5SQXFhaDR3QlFZREsyVncKQTBFQUtJU1FzN2VBT2dISDRndmo1U1BNMytvZnhNMDRYNjRtbVVudGIydDI3VDArK3lERXFJOVRXRW1XZHU4VwpOMGNOcFZXc3c3LytBd1hwSnlXdGNMZVhEdz09Ci0tLS0tRU5EIENFUlRJRklDQVRFLS0tLS0K","status":"SUCCESS","status_message":"SUCCESS"}
```

#### Getting certificate from CSR
```
$ curl http://127.0.0.1:8888/api/v1.0/create-from-csr -H "Content-Type: application/json" \ 
--request POST --data '{"csr_base64": "LS0tLS1CRUdJTiBDRVJUSUZJQ0FURSBSRVFVRVNULS0tLS0KTUlIbU1JR1pBZ0VBTUdZeEN6QUpCZ05WQkFZVEFsVlRNUTB3Q3dZRFZRUUlEQVJWZEdGb01RMHdDd1lEVlFRSApEQVJNWldocE1SWXdGQVlEVlFRS0RBMU9iM1J5WldGc0xDQkpibU11TVFzd0NRWURWUVFMREFKSlZERVVNQklHCkExVUVBd3dMYm05MGNtVmhiQzVqYjIwd0tqQUZCZ01yWlhBRElRQnczN0FKUFV4a0FpNktHNEpYUGxURlZYSzcKcXhJZURaMjJsWmcyTWNoQTY2QUFNQVVHQXl0bGNBTkJBRDlUTkowZTRSQ295NzFRbWhkc3hSTFhpZHR4K3kxVQpHbFlXU0NyeGlhSm1ZcHVlZ2o5NTRCRE1MNXlqWWdaRVpIT1dKQS9tS2x5eWQ4UGRpckIzd3dzPQotLS0tLUVORCBDRVJUSUZJQ0FURSBSRVFVRVNULS0tLS0K"}'
{"signed_cert_base64":"LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0tCk1JSUJtekNDQVUwQ0FRRXdCUVlESzJWd01Hb3hDekFKQmdOVkJBWVRBbFZUTVEwd0N3WURWUVFJREFSVmRHRm8KTVEwd0N3WURWUVFIREFSTVpXaHBNUmd3RmdZRFZRUUtEQTlPYjNSeVpXRnNZMkVzSUVsdVl5NHhDekFKQmdOVgpCQXNNQWtOQk1SWXdGQVlEVlFRRERBMXViM1J5WldGc1kyRXVZMjl0TUI0WERUSXlNVEV5TmpFM01UVXpNRm9YCkRUSXpNVEV5TmpFM01UVXpNRm93WmpFTE1Ba0dBMVVFQmhNQ1ZWTXhEVEFMQmdOVkJBZ01CRlYwWVdneERUQUwKQmdOVkJBY01CRXhsYUdreEZqQVVCZ05WQkFvTURVNXZkSEpsWVd3c0lFbHVZeTR4Q3pBSkJnTlZCQXNNQWtsVQpNUlF3RWdZRFZRUUREQXR1YjNSeVpXRnNMbU52YlRBcU1BVUdBeXRsY0FNaEFIRGZzQWs5VEdRQ0xvb2JnbGMrClZNVlZjcnVyRWg0Tm5iYVZtRFl4eUVEcm95RXdIekFkQmdOVkhRNEVGZ1FVMmptajdsNXJTdzB5VmIvdmxXQVkKa0svWUJ3a3dCUVlESzJWd0EwRUFHZ2Y0clRqcTNZVnV0NStvamtQenBLSGE0MmRWZDBvS3RkQTlTQzdYUjVZZgpqU2lST3pYYzdwZ2ltaGNoR3NXT05FSkxZR3VEMTQ2ZUx6T2FGRXlwRHc9PQotLS0tLUVORCBDRVJUSUZJQ0FURS0tLS0tCg==","status":"SUCCESS","status_message":"SUCCESS"}
```

## Cool readings
Things I've read for inspiration:
- https://blog.pinterjann.is/ed25519-certificates.html
- https://github.com/seanmonstar/warp/tree/master/examples
- https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/
- https://learn.microsoft.com/en-us/azure/iot-hub/tutorial-x509-openssl
- https://github.com/Azure/azure-iot-sdk-c/blob/main/tools/CACertificates/certGen.sh
- https://www.digicert.com/kb/ssl-support/openssl-quick-reference-guide.htm
