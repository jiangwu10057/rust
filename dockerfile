FROM rust:1.51
LABEL maintainer="jiangwu10057@qq.com" version="0.0.1"
WORKDIR /usr/src
RUN cargo new app
RUN app
RUN cargo run
# RUN cargo install --path .

# CMD ["app"]