FROM scratch
WORKDIR /app
ENV HOST 0.0.0.0
COPY --chmod=755 ./target/x86_64-unknown-linux-musl/release/action-demo .
EXPOSE 8360
ENTRYPOINT ["./action-demo"]
