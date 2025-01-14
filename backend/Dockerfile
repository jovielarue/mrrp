FROM rust:latest

# Working dir inside the container
WORKDIR /app

# Install the app on path
COPY . .
RUN cargo run

CMD ["portable-media"]
