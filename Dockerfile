# based on
# https://kerkour.com/rust-small-docker-image

####################################################################################################
## Cargo Chef
####################################################################################################
FROM rust:latest AS chef

RUN cargo install cargo-chef 

WORKDIR /app

FROM chef AS planner

COPY . .

RUN cargo chef prepare  --recipe-path recipe.json

####################################################################################################
## Builder
####################################################################################################
FROM chef AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=nonroot
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch AS runtime

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/to_do-axtix ./

# Use an unprivileged user.
USER nonroot:nonroot

CMD ["/app/to_do-axtix"]
