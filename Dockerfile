FROM rust:stretch

RUN apt update && apt upgrade -y && apt install -y libasound2-dev gstreamer1.0 cmake

WORKDIR /src

COPY ./ /src/

RUN cargo build --release

ENTRYPOINT /src/target/release/rustic
