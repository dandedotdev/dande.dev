FROM rustlang/rust:nightly-bookworm AS builder

RUN apt-get update && apt-get install -y nodejs npm

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

RUN cargo binstall cargo-leptos -y

RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

RUN npm install

RUN cargo update
RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-bookworm AS runtime

WORKDIR /app
COPY --from=builder /app/.data /app/.data
COPY --from=builder /app/public /app/public
COPY --from=builder /app/target/release/app /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
# should be aligned with `package.json`
ENV LEPTOS_TAILWIND_VERSION = "4.1.4"
ENV RUST_LOG="info"

EXPOSE 8080

CMD [ "/app/app" ]
