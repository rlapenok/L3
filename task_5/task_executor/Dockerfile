FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

#create workdir
WORKDIR /task_executor
#copy files
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian12

COPY --from=build /usr/local/cargo/bin/task_executor .
COPY --from=build /task_executor/config.toml  .

CMD ["./task_executor", "-a" "-p", "config.toml"]
