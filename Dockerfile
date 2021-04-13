FROM rust:1.51.0 as build

WORKDIR /usr/src/actix-covid19
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim

COPY --from=build /usr/local/cargo/bin/actix-covid19 /usr/local/bin/actix-covid19

ENTRYPOINT [ "actix-covid19" ]