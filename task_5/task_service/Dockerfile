FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

#create workdir
WORKDIR /task_service
#copy files
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian12

COPY --from=build /usr/local/cargo/bin/task_service .
COPY --from=build /task_service/migrations  ./migrations
COPY --from=build /task_service/config.toml  .

CMD ["./task_service", "-a", "0.0.0.0:8080", "-p", "config.toml"]
