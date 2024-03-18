FROM rust:latest as build

WORKDIR /

RUN apt-get update \
 && apt-get -y install curl build-essential clang pkg-config libjpeg-turbo-progs libpng-dev gifsicle webp \
 && rm -rfv /var/lib/apt/lists/*

ENV MAGICK_VERSION 7.1.1-29

RUN curl https://imagemagick.org/archive/ImageMagick-${MAGICK_VERSION}.tar.gz | tar xz \
 && cd ImageMagick-${MAGICK_VERSION} \
 && ./configure --with-magick-plus-plus=no --with-perl=no \
 && make \
 && make install \
 && cd .. \
 && rm -r ImageMagick-${MAGICK_VERSION}*

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

