FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /x509-parser
WORKDIR /x509-parser/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /x509-parser/fuzz/target/x86_64-unknown-linux-gnu/release/x509-parser-fuzz /