# Create the build container to compile the hello world program
FROM ekidd/rust-musl-builder:latest as builder
# ENV USER root
ADD --chown=rust:rust . ./
COPY . .

RUN cargo build --release --features=rdkafka/cmake_build

# Create the execution container by copying the compiled hello world to it and running it
FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/queubious /queubious
COPY --from=builder /home/rust/src/templates /templates
COPY --from=builder /home/rust/src/build /templates
CMD ["/queubious"]