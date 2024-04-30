# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

ARG RUST_VERSION=1.76.0
ARG APP_NAME=morningexapiproduction

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-alpine3.19 AS build
ENV DATABASE_URL=${DATABASE_URL}
ENV CLIENT_ID=${CLIENT_ID}
ENV CLIENT_SECRET=${CLIENT_SECRET}
ENV JWT_SECRET=${JWT_SECRET}
ENV HOST="0.0.0.0"
ENV PORT="8080"

ARG APP_NAME
WORKDIR /app
RUN export DOCKER_DEFAULT_PLATFORM=linux/amd64
# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git
RUN apk update && apk add strace
RUN apk add --no-cache pkgconf openssl-dev musl-dev cmake make gcc g++ nodejs perl clang16 curl strace
RUN export DOCKER_BUILDKIT=1
ENV OPENSSL_DIR=/usr

# Copy source code and dependencies manifests into the container.
COPY src /app/src
COPY migrations /app/migrations
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock

# Build the application.
# Leveraging the volumes directly without BuildKit's --mount.
RUN cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application.

FROM alpine:3.18 AS final

# Create a non-privileged user that the app will run under.
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 8080

# What the container should run when it is started.
CMD ["/bin/server"]
