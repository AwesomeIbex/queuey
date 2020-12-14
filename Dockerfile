# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

WORKDIR /usr/src/queuey

COPY Cargo.toml Cargo.toml

RUN mkdir src/
RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/queuey*

COPY . .

RUN cargo build --release

RUN cargo install --path .iclo

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

COPY --from=cargo-build /usr/local/cargo/bin/queuey /usr/local/bin/queuey

#TODO add commands
CMD ["queuey"]