FROM ubuntu:22.04

WORKDIR /work/wasm

RUN apt-get update && apt-get install -yq \
    curl \
    libssl-dev \
    pkg-config \
    build-essential \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*


RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH=/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh


COPY Cargo.toml Cargo.lock ./
COPY ./src src
COPY ./tests tests

CMD wasm-pack build
