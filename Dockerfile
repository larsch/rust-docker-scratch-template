FROM alpine as builder
RUN apk update
RUN apk add rustup build-base musl-dev perl
RUN rustup-init -y
ENV PATH=/root/.cargo/bin:$PATH
WORKDIR /src
COPY . .
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release

FROM scratch
COPY --from=builder /src/target/release/rust-docker-scratch-template /app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
ENTRYPOINT ["/app"]
