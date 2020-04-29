FROM rust:1.43 as builder
COPY . .
RUN cargo build --release
FROM rust:1.43
RUN mkdir image_uploader
COPY --from=builder /target/release/* ./image_uploader/
EXPOSE 3000
ENTRYPOINT ["/image_uploader/image-uploader"]
