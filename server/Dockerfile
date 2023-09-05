FROM rust:1.69-alpine as builder
WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl
RUN apk add build-base openssl openssl-dev
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

RUN USER=root cargo new dawnofwar
WORKDIR /app/dawnofwar
RUN cargo new --lib database
RUN cargo new --lib ratings
RUN mkdir -p migrations
COPY Cargo.* ./
COPY ./database/Cargo.* ./database/
RUN echo 'fn main() { println!("BUILT"); }' > ./src/main.rs
RUN cargo build --release

COPY database ./database
COPY ratings ./ratings
COPY migrations ./migrations
COPY src ./src
RUN rm -rf /app/dawnofwar/target/release
COPY ./src/main.rs ./src/main.rs
RUN touch ./src/main.rs
RUN cargo build --release
RUN cargo build --release -p ratings

FROM scratch as runtime

COPY --from=builder /app/dawnofwar/target/release/rust-dawnofwar .
COPY --from=builder /app/dawnofwar/target/release/ratings .
EXPOSE 3000
USER 1000
CMD ["./rust-dawnofwar"]
