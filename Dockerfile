FROM rust:buster

RUN apt update && apt upgrade -y && apt install -y libasound2-dev libgstreamer1.0-dev cmake

WORKDIR /src

COPY ./ /src/

RUN cargo build --release

ENTRYPOINT /src/target/release/rustic
