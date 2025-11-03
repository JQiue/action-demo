FROM scratch
WORKDIR /app
ENV HOST 0.0.0.0
COPY ./target/x86_64-unknown-linux-musl/release/action-demo .
EXPOSE 8360
ENTRYPOINT ["./action-demo"]
