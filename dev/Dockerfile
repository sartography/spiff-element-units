FROM rust:1-slim AS base

RUN \
    --mount=type=cache,target=/var/cache/apt \
    apt-get update \
    && apt-get clean -y \
    && apt-get install -y -q \
       python3 python3-pip python3-venv python-is-python3

RUN rustup component add rustfmt
RUN python -m pip install --upgrade pip

COPY dev/requirements.txt .

RUN \
    --mount=type=cache,target=/var/cache/pip \
    pip install -r requirements.txt

WORKDIR /module

COPY module/ ./

RUN python -m venv .env

RUN \
    --mount=type=cache,target=/var/cache/cargo \
    cargo build