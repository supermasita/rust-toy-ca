# Example:                                                                                                                                                                               
# 
# $ sudo docker build --tag "supermasita/rust-toy-ca" .
# $ sudo docker run -v /home/user/certs/:/certs/ -p 8888:8888 --rm supermasita/rust-toy-ca /usr/bin/rust-toy-ca --ca-cert-file /certs/ca.crt --ca-pkey-file /certs/ca.key --listen 0.0.0.0:8888 

FROM ubuntu:latest

ENV UPDATED "2025-03-29"

# Install dependencies
RUN apt-get update \
    && apt-get install -y curl unzip openssl libssl-dev pkg-config build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set the PATH environment variable to include Cargo's bin directory
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory
WORKDIR /opt/

# Download and unzip the rust-toy-ca project
RUN curl -L https://codeload.github.com/supermasita/rust-toy-ca/zip/main -o main.zip \
    && unzip main.zip \
    && mv rust-toy-ca-main rust-toy-ca \
    && rm -f main.zip

# Build the project
RUN cd rust-toy-ca \
    && cargo build --release \
    && cp -a target/release/rust-toy-ca /usr/bin/ \ 
    && cargo clean

# Expose the port
EXPOSE 8888
