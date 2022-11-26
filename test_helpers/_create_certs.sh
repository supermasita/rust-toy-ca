#!/bin/bash

# Use `-c y` if you need to create the CA pk/cert
while getopts c: option
do
    case "${option}"
        in
        c)CREATE_CA=${OPTARG};;
    esac
done

CA_CRT="ca.crt"
CA_CSR="ca.csr"
CA_KEY="ca.key"
CA_SUBJ="/C=US/ST=Utah/L=Lehi/O=Notrealca, Inc./OU=CA/CN=notrealca.com"
REQUESTER_CRT="requester.crt"
REQUESTER_CSR="requester.csr"
REQUESTER_KEY="requester.key"
REQUESTER_SUBJ="/C=US/ST=Utah/L=Lehi/O=Notreal, Inc./OU=IT/CN=notreal.com"

if [[ $CREATE_CA == "y" ]]; then
    echo "CREATING CA..."
    openssl genpkey -algorithm ED25519 > $CA_KEY
    openssl req -new -out $CA_CSR -key $CA_KEY -subj "$CA_SUBJ"
    openssl x509 -req -days 700 -in $CA_CSR -signkey $CA_KEY -out $CA_CRT
fi

echo "CREATING REQUESTER..."
openssl genpkey -algorithm ED25519 > $REQUESTER_KEY
openssl req -new -out $REQUESTER_CSR -key $REQUESTER_KEY -subj "$REQUESTER_SUBJ" 
openssl x509 -req -days 700 -in $REQUESTER_CSR -out $REQUESTER_CRT -CA $CA_CRT -CAkey $CA_KEY -CAcreateserial

echo "VERIFY..."
openssl verify -verbose -CAfile $CA_CRT $REQUESTER_CRT