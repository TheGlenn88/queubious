# Create the build container to compile the program
FROM ekidd/rust-musl-builder:latest as builder
# ENV USER root
ADD --chown=rust:rust . ./
COPY . .

RUN cargo build --release --features=rdkafka/cmake_build

# Create the execution container by copying the compiled binary and template files
FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/queubious /queubious
COPY --from=builder /home/rust/src/templates /templates
COPY --from=builder /home/rust/src/build /build
COPY --from=builder /home/rust/src/.env /.env
CMD ["/queubious"]