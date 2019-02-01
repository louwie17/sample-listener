FROM arm64v8/rust:slim

RUN apt-get update &&\
    apt-get -y upgrade &&\
    apt-get -y install wget curl make gcc libsqlite3-dev
RUN apt-get clean

RUN rustup target install aarch64-unknown-linux-gnu

# Create app directory
WORKDIR /usr/project

CMD ["/usr/bin/make -C sample-listener"]