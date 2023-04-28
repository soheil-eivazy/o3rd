FROM rust:latest as build


WORKDIR /usr/src/fullstack/
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

COPY --from=build usr/src/fullstack/target/release/nucleus /use/local/bin/nucleus

WORKDIR /usr/local/bin
CMD ["nucleus"]