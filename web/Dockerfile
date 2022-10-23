FROM rust:1.64

WORKDIR /usr/app/
COPY "./src/" ./src/
COPY "./Cargo.lock" ./
COPY "./Cargo.toml" ./
COPY "./.env" ./

RUN cargo build --release 
VOLUME "/usr/app/target"

CMD ["cargo", "run"]