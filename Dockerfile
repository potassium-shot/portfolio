FROM lewimbes/dioxus:0.7.5 AS builder
WORKDIR /usr/src/portfolio
COPY . .
RUN dx bundle --release

FROM debian:oldstable-20260316-slim
RUN mkdir /usr/local/bin/portfolio
COPY --from=builder /usr/src/portfolio/target/dx/portfolio/release/web/server /usr/local/bin/portfolio
COPY --from=builder /usr/src/portfolio/target/dx/portfolio/release/web/public /usr/local/bin/portfolio/public
ENV IP="0.0.0.0"
CMD ["/usr/local/bin/portfolio/server"]

