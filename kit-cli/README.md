# Kit CLI

A CLI tool for scaffolding Kit web applications.

## Installation

```bash
cargo install kit-cli
```

## Usage

### Create a new project

```bash
kit new myapp
```

This will interactively prompt you for:
- Project name
- Description
- Author

### Non-interactive mode

```bash
kit new myapp --no-interaction
```

### Skip git initialization

```bash
kit new myapp --no-git
```

## Generated Project Structure

```
myapp/
├── Cargo.toml
├── .gitignore
└── src/
    ├── main.rs
    └── controllers/
        ├── mod.rs
        └── home.rs
```

## License

MIT
