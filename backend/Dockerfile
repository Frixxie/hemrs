FROM rust:1.74.0
WORKDIR /usr/src/wictk
ENV DATABASE_URL=sqlite://sqlite.db
COPY . .
RUN cargo install sqlx-cli
RUN sqlx db create
RUN sqlx migrate run
RUN cargo install --path .
CMD ["backend"]

