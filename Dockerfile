FROM rust:latest as builder


LABEL "maintainer"="Oto Brglez <otobrglez@gmail.com>"
LABEL "org.opencontainers.image.url"="https://github.com/ogrodje/podcasters-collector"
LABEL "org.opencontainers.image.source"="https://github.com/ogrodje/podcasters-collector"

WORKDIR /usr/src/myapp

COPY . .

RUN apt update && apt install musl-tools -y && \
  rustup target add x86_64-unknown-linux-musl && \
  cargo build \
    --target x86_64-unknown-linux-musl \
    --release

CMD ["/usr/src/myapp/target/x86_64-unknown-linux-musl/release/podcasters-collector"]

FROM alpine AS runtime
LABEL "maintainer"="Oto Brglez <otobrglez@gmail.com>"
LABEL "org.opencontainers.image.url"="https://github.com/ogrodje/podcasters-collector"
LABEL "org.opencontainers.image.source"="https://github.com/ogrodje/podcasters-collector"

RUN apk --no-cache add ca-certificates
COPY --from=builder /usr/src/myapp/target/x86_64-unknown-linux-musl/release/podcasters-collector /usr/local/bin/
CMD ["podcasters-collector"]

#
#RUN apk --no-cache add ca-certificates
#COPY --from=builder /usr/src/myapp/target/release/podcasters-collector /usr/local/bin
#CMD ["/usr/local/bin/podcasters-collector"]
