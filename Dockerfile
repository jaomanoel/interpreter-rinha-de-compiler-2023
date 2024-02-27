# Stage 1: Build the Rust project
FROM rust:1-slim-buster AS build

WORKDIR /app/tree-walker-interpreter

# Copy only the Cargo.toml and Cargo.lock files to capture dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy src folder to avoid errors during the build
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the project (Cargo.toml is needed here)
RUN cargo build --release

# Stage 2: Create the final image
FROM rust:1-slim-buster

WORKDIR /app/tree-walker-interpreter

# Copy only the built artifacts from the previous stage
COPY --from=build /app/tree-walker-interpreter/target/release/ /app/tree-walker-interpreter

# Install Rinha
RUN cargo install rinha

# Copy the rest of the project files
COPY . .

# Convert Rinha source files to JSON
RUN for file in /app/tree-walker-interpreter/examples/*.rinha; do \
  output_file="/app/tree-walker-interpreter/examples/$(basename "$file" .rinha).json"; \
  rinha "$file" > "$output_file"; \
  done

# CMD to run the interpreter
CMD ["cargo", "run"]
