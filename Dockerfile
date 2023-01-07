
FROM rust:latest
COPY rust /rust
WORKDIR /rust
RUN cargo build --release

FROM --platform=linux/x86_64 openresty/openresty:alpine-fat

RUN apk add gcompat 
RUN set LD_LIBRARY_PATH="/lib;/lib64"

COPY ./default.conf /etc/nginx/conf.d/
RUN mkdir /etc/nginx/lua && chmod -R 777 /etc/nginx/lua
COPY ./access.lua /etc/nginx/lua
COPY --from=0 /rust/target/release/librustjwt.so /etc/nginx/lua

