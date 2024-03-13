# Example:
# 
# $ sudo docker build --tag "supermasita/rust-toy-ca" .
# $ sudo docker run -v /home/user/certs/:/certs/ -p 8888:8888 --rm supermasita/rust-toy-ca /usr/bin/rust-toy-ca --ca-cert-file /certs/ca.crt --ca-pkey-file /certs/ca.key --listen 0.0.0.0:8888 

FROM ubuntu:latest

ENV UPDATED "2024-03-13"

RUN apt-get update \
    && apt-get install -y cargo curl unzip openssl libssl-dev pkg-config

RUN cd /opt/ \
    && curl https://codeload.github.com/supermasita/rust-toy-ca/zip/main -o main.zip \
    && unzip main.zip \
    && mv rust-toy-ca-main rust-toy-ca \
    && rm -f main.zip

RUN cd /opt/rust-toy-ca \
    && cargo build --release \
    && cp -a target/release/rust-toy-ca /usr/bin/ \
    && cargo clean

EXPOSE 8888
