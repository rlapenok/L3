FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

#create workdir
WORKDIR /task_notifier
#copy files
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian12

COPY --from=build /usr/local/cargo/bin/task_notifier .
COPY --from=build /task_notifier/config.toml  .

CMD ["./task_notifier", "-a", "0.0.0.0:6060", "-p", "config.toml"]