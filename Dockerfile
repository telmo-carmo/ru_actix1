FROM rust:1-slim-alpine
# Using a slim image significantly reduces the base image size.


# Set the working directory
WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo install --path .  --release 

COPY . .

CMD ["cargo", "run" "--release"]
#CMD ["./target/release/<your_binary_name>"] 


# docker build -t my-rust-app . 
# docker run -p 8080:8080 my-rust-app
