FROM rust:latest

# Working dir inside the container
WORKDIR /app

# Copy all files into container
COPY . .

RUN cargo install --path .

CMD ["portable-media"]
