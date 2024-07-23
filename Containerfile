# Get started with a build env with Rust nightly
FROM docker.io/rustlang/rust:nightly-alpine as builder

RUN apk update && apk add --no-cache wget libc-dev

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --release -vv

########################################

FROM docker.io/rustlang/rust:nightly-alpine as runner

WORKDIR /app

COPY --from=builder /work/target/release/server /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site/beu"

EXPOSE 3000

CMD ["/app/server"]
