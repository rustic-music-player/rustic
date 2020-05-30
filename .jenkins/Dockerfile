FROM rust:buster

RUN apt update && apt upgrade -y && apt install -y libasound2-dev cmake libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libgstreamer-plugins-bad1.0-dev libdbus-1-dev dbus qtbase5-dev qtdeclarative5-dev
RUN cargo install sccache
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/build_cache/sccache
