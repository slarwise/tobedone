FROM rust:1.67

WORKDIR /usr/src/items
COPY . .

RUN cargo install --path .

CMD ["items"]
