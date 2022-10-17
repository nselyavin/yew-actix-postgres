FROM rust:1.64

WORKDIR /usr/app/
COPY ./ ./

RUN cargo build --release 
VOLUME "/usr/app/target"

EXPOSE 8080
CMD ["cargo", "run"]