FROM ubuntu:jammy AS builder

# Install dependencies
RUN apt-get update && apt-get install -y curl libpq5 libpq-dev build-essential
RUN apt-get update && apt-get install -y software-properties-common python3 pkg-config

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain nightly

ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./

RUN cargo build --release

FROM ubuntu:jammy

# Install dependencies
RUN apt-get update && apt-get install -y libpq5 libpq-dev 
RUN apt-get update && apt-get install -y sudo 
RUN apt-get update && apt-get install -y software-properties-common python3 pkg-config

# Setup sudo
RUN useradd -m docker && echo "docker:docker" | chpasswd && adduser docker sudo

# Install binary
COPY --from=builder \
  /target/release/tool-app \
  /usr/local/bin/

ENV PORT 8080

WORKDIR /root
CMD ROCKET_PORT=$PORT /usr/local/bin/tool-app