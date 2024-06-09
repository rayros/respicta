FROM rust:slim-bookworm as base

WORKDIR /

RUN apt-get update \
 && apt-get -y install nasm curl build-essential cmake clang pkg-config libjpeg-turbo-progs libjpeg-dev libpng-dev gifsicle webp libwebp-dev libssl-dev \
 && rm -rfv /var/lib/apt/lists/*

ENV MAGICK_VERSION 7.1.1-33

RUN curl https://imagemagick.org/archive/ImageMagick-${MAGICK_VERSION}.tar.gz | tar xz \
 && cd ImageMagick-${MAGICK_VERSION} \
 && ./configure --with-magick-plus-plus=no --with-perl=no \
 && make \
 && make install \
 && cd .. \
 && rm -r ImageMagick-${MAGICK_VERSION}*

ENV LD_LIBRARY_PATH=/usr/local/lib

FROM base as build

RUN cargo new app

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./

RUN cargo build

COPY ./src ./src

RUN cargo build

FROM build as test

RUN cargo install cargo-nextest --locked

COPY ./examples ./examples
COPY ./tests ./tests

RUN cargo nextest run --all-features

FROM build as release

RUN cargo build --release --features=cli

FROM base as checks

RUN cargo install cargo-semver-checks cargo-audit cargo-outdated cargo-nextest --locked

WORKDIR /app

COPY . ./

RUN cargo outdated --exit-code 1

RUN cargo nextest run --all-features

RUN cargo semver-checks

RUN cargo audit

FROM base as publish

RUN cargo install cargo-semver-checks --locked

WORKDIR /publish

COPY . ./

RUN --mount=type=secret,id=CARGO_REGISTRY_TOKEN \
   export CARGO_REGISTRY_TOKEN=$(cat /run/secrets/CARGO_REGISTRY_TOKEN) \
   && cargo semver-checks \
   && cargo publish

FROM debian:bookworm-slim

RUN apt-get update \
 && apt-get -y install libjpeg-turbo-progs libjpeg-dev libpng-dev gifsicle webp libgomp1 \
 && rm -rfv /var/lib/apt/lists/*

COPY --from=release /usr/local/lib /usr/local/lib

COPY --from=release /app/target/release/respicta /usr/local/bin/respicta

ENV LD_LIBRARY_PATH=/usr/local/lib

WORKDIR /images

# smoke test
RUN respicta --help

ENTRYPOINT ["/bin/bash", "-c", "respicta \"$@\"", "--"]
