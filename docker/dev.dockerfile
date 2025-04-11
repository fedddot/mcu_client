FROM rust:latest AS builder

RUN apt-get update
RUN apt-get install -y libudev-dev

RUN rustup component add clippy-preview

# Sources root dir should be mounted to this location when running the container
WORKDIR /usr/app/src

CMD ["/bin/bash", "-c"]