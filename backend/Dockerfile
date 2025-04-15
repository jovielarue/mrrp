FROM rust:latest

RUN apt-get update && apt-get install -y cmake
# Working dir inside the container
WORKDIR /app

# Install the app on path
COPY . .

CMD ["cargo", "update"]
CMD ["cargo", "run"]
