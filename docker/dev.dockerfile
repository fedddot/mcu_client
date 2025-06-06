FROM rust:slim-bookworm AS dev-img

RUN apt-get update
RUN apt-get install -y libudev-dev pkg-config
RUN apt-get install -y git
RUN apt-get install -y protobuf-compiler

RUN rustup component add clippy-preview

ENV CARGO_CARGO_NEW_VCS=none

# Sources root dir should be mounted to this location when running the container
WORKDIR /usr/app/src

CMD ["/bin/bash", "-c"]