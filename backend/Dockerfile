FROM rust:latest

# Working dir inside the container
WORKDIR /app

# Install the app on path
COPY . .

CMD ["cargo", "run", ">", "dev/null"]
