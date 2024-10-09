FROM alpine:3.19

# Install dependencies

RUN apk add musl-dev openssl-dev clang-dev
RUN apk add rust cargo

## add user with UID 1000 and GID 1000

RUN addgroup -g 1000 rust && \
    adduser -D -u 1000 -G rust rust

USER rust

WORKDIR /home/rust/rs
