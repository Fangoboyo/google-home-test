# Use an official Rust runtime as a parent image
FROM rust:1.54

RUN rustup default nightly

# Set the working directory in the container to /usr/src/GHBackend
WORKDIR /usr/src/GHBackend

# Copy the current directory contents into the container at /usr/src/GHBackend
COPY . .
# Install any needed packages specified in Cargo.toml
RUN cargo install --path .

# Make port 8000 available to the world outside this container
EXPOSE 80

# Run the binary program produced by `cargo install`
CMD ["GHBackend"]