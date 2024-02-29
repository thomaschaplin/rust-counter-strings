####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev binutils

# Create appuser
ENV USER=rust
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /rust-counter-strings

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

RUN strip target/x86_64-unknown-linux-musl/release/rust-counter-strings

# Download upx binary
ADD https://github.com/upx/upx/releases/download/v4.2.2/upx-4.2.2-amd64_linux.tar.xz /tmp/upx.tar.xz
RUN tar -xvf /tmp/upx.tar.xz -C /tmp && cp /tmp/upx-4.2.2-amd64_linux/upx /usr/local/bin/upx

####################################################################################################
## Final Image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /rust-counter-strings

# Copy our build
COPY --from=builder /rust-counter-strings/target/x86_64-unknown-linux-musl/release/rust-counter-strings ./

# Use an unprivileged user.
USER rust:rust

ENTRYPOINT ["./rust-counter-strings"]
