# Getting Started

## Overview

This repository contains the source code of my interpreter implementation for [Rinha Compilers](https://github.com/aripiprazole/rinha-de-compiler), a tool developed to analyze and interpret the Rinha language. Follow the steps below to get started.

## Prerequisites

- [Git](https://git-scm.com/)
- [Rust](https://www.rust-lang.org/)

## Installation

Clone the repository to your local machine:

```bash
git clone https://github.com/jaomanoel/interpreter-rinha-de-compiler-2023.git
cd interpreter-rinha-de-compiler-2023
```

Install the Rinha using Cargo:

```bash 
cargo install rinha
```

## Usage

Navigate to the tree-walker-interpreter directory:

```bash
cd tree_walker_interpreter
```

Convert Rinha source files to JSON:

```bash
for file in examples/*.rinha; do
    output_file="examples/$(basename "$file" .rinha).json"
    rinha "$file" | jq > "$output_file"
done
```

You can use the provided examples in the examples directory. The Rinha source files will be converted to JSON, and the interpreter will run the converted files.

Run the interpreter:

```bash
cargo run
```

## License

This project is licensed under the [MIT License](https://opensource.org/license/mit/).


