FROM rust:1.56.1-slim

RUN  mkdir /usr/src/myapp \
    mkdir /usr/src/myapp/src
WORKDIR /usr/src/myapp
COPY Cargo.toml /usr/src/myapp
COPY src/* /usr/src/myapp/src
RUN cargo install --path .
CMD ["actix-test"]