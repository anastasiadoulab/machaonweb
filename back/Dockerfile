FROM rust:1.67

ADD ./src /opt/machaonweb_back/src
ADD ./protos /opt/machaonweb_back/protos
ADD ./migrations /opt/machaonweb_back/migration
ADD Cargo.toml Cargo.lock build.rs diesel.toml /opt/machaonweb_back/

WORKDIR /opt/machaonweb_back

RUN apt-get update \
&& apt-get upgrade -y \
&& apt-get install -y protobuf-compiler libprotobuf-dev libmariadb-dev-compat libmariadb-dev \
&& cargo build --release
