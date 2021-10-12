FROM ekidd/rust-musl-builder:stable as builder
COPY ./Cargo.toml /home/rust/src/Cargo.toml
COPY ./Cargo.lock /home/rust/src/Cargo.lock
ADD ./jsonrpc-proto /home/rust/src/jsonrpc-proto/
ADD ./jsonrpc-app /home/rust/src/jsonrpc-app/
ADD ./jsonrpc-key /home/rust/src/jsonrpc-key/
ADD ./jsonrpc-gw /home/rust/src/jsonrpc-gw/
RUN cargo build --release
RUN ls -All /home/rust/src/target/

FROM alpine:latest
EXPOSE 8000
ENV TZ=Etc/UTC \
    APP_USER=appuser \
    RUST_BACKTRACE=1
RUN addgroup -S $APP_USER && adduser -S -g $APP_USER $APP_USER
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/jsonrpc-app /usr/src/app/jsonrpc-app
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/jsonrpc-key /usr/src/app/jsonrpc-key
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/jsonrpc-gw /usr/src/app/jsonrpc-gw
RUN chown -R $APP_USER:$APP_USER /usr/src/app
USER $APP_USER
WORKDIR /usr/src/app
ENTRYPOINT ["/usr/src/app/jsonrpc-gw"]

