FROM rust:buster

RUN apt update && apt upgrade -y && apt install -y libasound2-dev cmake libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libgstreamer-plugins-bad1.0-dev libdbus-1-dev dbus qtbase5-dev qtdeclarative5-dev libgtk-3-dev
RUN cargo install sccache
RUN cargo install cargo-tarpaulin
ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
ENV SCCACHE_DIR=/build_cache/sccache
