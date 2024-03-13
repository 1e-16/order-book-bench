# 设置基础镜像为 alpine
FROM alpine:latest as base

RUN apk add --no-cache curl build-base
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache libgcc
COPY --from=base /app/target/release/order-book-bench /usr/local/bin/
CMD ["order-book-bench"]

LABEL name="order-book-bench"