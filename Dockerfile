FROM nvidia/cuda:12.2.2-devel-ubuntu20.04 as base

ENV DEBIAN_FRONTEND=noninteractive

RUN apt update && apt -y install curl libssl-dev pkg-config
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN curl -L https://risczero.com/install | bash
ENV PATH="$PATH:/root/.risc0/bin"
RUN rzup install

WORKDIR /src

FROM base AS builder
COPY . .
RUN --mount=type=cache,id=release,target=/root/.cache \
  --mount=type=cache,id=release,target=/root/.cargo,from=base,source=/root/.cargo \
cargo build --release --features cuda
