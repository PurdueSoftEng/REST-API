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

RUN apt-get update && apt-get install -y libmysqlclient-dev sudo mysql-server systemctl

RUN useradd -m docker && echo "docker:docker" | chpasswd && adduser docker sudo

RUN grep -v bind-address /etc/mysql/my.cnf > temp.txt && mv temp.txt /etc/mysql/my.cnf

COPY --from=builder \
  /target/release/tool-app \
  /usr/local/bin/

ENV PORT 8080

EXPOSE 8080

WORKDIR /root
CMD /etc/init.d/mysql start && echo "CREATE USER 'rocket'@'%' IDENTIFIED BY 'password'; CREATE DATABASE app; GRANT ALL PRIVILEGES ON *.* TO 'rocket' WITH GRANT OPTION;" | sudo mysql && ROCKET_PORT=$PORT /usr/local/bin/tool-app