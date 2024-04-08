FROM rust:slim-bookworm as base

WORKDIR /

RUN apt-get update \
 && apt-get -y install curl build-essential cmake clang pkg-config libjpeg-turbo-progs libjpeg-dev libpng-dev gifsicle webp libssl-dev \
 && rm -rfv /var/lib/apt/lists/*

ENV MAGICK_VERSION 7.1.1-30

RUN curl https://imagemagick.org/archive/ImageMagick-${MAGICK_VERSION}.tar.gz | tar xz \
 && cd ImageMagick-${MAGICK_VERSION} \
 && ./configure --with-magick-plus-plus=no --with-perl=no \
 && make \
 && make install \
 && cd .. \
 && rm -r ImageMagick-${MAGICK_VERSION}*

FROM base as build

RUN cargo new app

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./

RUN cargo build

COPY ./src ./src

RUN cargo build

ENV LD_LIBRARY_PATH=/usr/local/lib

FROM build as test

COPY ./tests ./tests

RUN cargo test

FROM build as release

RUN cargo build --release

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
RUN respicta

ENTRYPOINT ["/bin/bash", "-c", "respicta \"$@\"", "--"]
