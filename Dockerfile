FROM rust:latest

WORKDIR /usr/src/ticker
COPY . .

RUN cargo install --path .
# RUN cargo build --release

CMD [ "ticker" ]
