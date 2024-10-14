# Contributing to Scrounch Backend

First off, thank you for considering contributing to Scrounch Backend! 🎉

The following guidelines will help you through the process of contributing effectively to the project.

## Table of Contents
- [Getting Started](#getting-started)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)
- [Development Workflow](#development-workflow)
- [Pull Request Process](#pull-request-process)
- [Documentation](#documentation)

## Getting Started

1. Fork the repository and clone it locally:
```bash
git clone https://github.com/your-username/scrounch-backend.git
```

### Install dependencies:
Ensure you have Rust and Cargo installed or use [Nix Package Manager](https://nixos.org/)
```bash
nix develop
``` 

### Set up the project:
Ensure you have a running instance of:
- Cache: Redis/Valkey, 
- An Openid Provider (Keycloak by example) for authentication
- An S3 Provider (MinIO by example).

You can use the example Docker Compose or any other preferred method to start these services. 
Example:
```sh
docker-compose up -d
```

### Run the project:
```sh
cargo run
```

## Reporting Bugs:
If you find a bug, please create a GitHub Issue with detailed steps to reproduce the problem, along with logs or screenshots if applicable. Make sure to include:
- Rust version (`rustc --version`)
- Operating system
- Steps to reproduce the bug

## Suggesting Features
We appreciate feature suggestions! If you have an idea, open a new GitHub issue and use the feature request template. Please describe:

What the feature would do
Why it would be useful
Any potential impact on the current codebase

## Development Workflow
Create a new branch for each feature or bugfix:
```sh
git checkout -b feature/my-awesome-feature
``` 

Ensure all tests pass before submitting a pull request:
```sh
cargo test --all-features --workspace
```

### Commits: 
- Keep commits small and focused.
- Write meaningful commit messages. 
- Use [Conventionnal Commit](https://www.conventionalcommits.org/en/v1.0.0/).

```
git commit -m "feat(auth): Add user authentication middleware"
```

Push your changes to your fork and open a pull request.

> [!WARNING]
> I won't accept any commit containing a .vscode, .idea, etc... folder/file, i'm happy that you love your IDE / Text Editor but please keep it (and it's files) for yourself


### Coding Guidelines

Follow the Rust style guide.

Ensure all functions, modules, and files have appropriate documentation using:
```
    //! for file-level comments
    /// for function or module documentation
```

Code and Comments should be written in English.

### Testing

Tests are crucial to maintain the stability of the project. Make sure to:

Write unit tests for any new functionality using the built-in Rust testing framework.
Use testcontainers for integration testing if your changes interact with external services like Redis or databases.
Run the test suite before submitting your PR:

```sh
cargo test --all-features --workspace
``` 

## Pull Request Process

Ensure your code adheres to the coding guidelines and is properly tested.
Update the README or documentation if your changes impact the API.
Open a pull request, describe your changes, and reference any related issues.
One of the maintainers will review your PR. Make sure to address any requested changes.

## Documentation

Run the following command to generate the Rust documentation:
```bash
cargo doc --document-private-items --open
```

You can also access the interactive API documentation (Swagger UI) when running the application in debug mode.
Open your browser and go to: http://localhost:3000/swagger-ui/

