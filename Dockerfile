FROM rust:1.83.0-slim as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/config config
COPY --from=builder /usr/src/src/fixtures src/fixtures
COPY --from=builder /usr/src/target/release/loco_inventory-cli loco_inventory-cli


CMD ["sh", "-c", "./loco_inventory-cli db migrate --environment production; ./loco_inventory-cli db seed --environment production; ./loco_inventory-cli start --environment production --binding 0.0.0.0"]
