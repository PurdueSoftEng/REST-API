FROM ubuntu:jammy AS builder

# You'll need to change `libmysqlclient-dev` to `libpq-dev` if you're using Postgres
RUN apt-get update && apt-get install -y curl libmysqlclient-dev build-essential

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain nightly

ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./

RUN cargo build --release

FROM ubuntu:jammy

RUN apt-get update && apt-get install -y libmysqlclient-dev sudo

RUN useradd -m docker && echo "docker:docker" | chpasswd && adduser docker sudo

COPY --from=builder \
  /target/release/tool-app \
  /usr/local/bin/

WORKDIR /root
CMD ROCKET_PORT=8080 /usr/local/bin/tool-app