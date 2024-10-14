# Scrounch Backend
> By Florian 'FloRide' Reimat

## About
This project is a backend API built using Rust with the Axum web framework, designed to handle operations for an e-commerce-like platform. The core functionalities include product management (CRUD operations), file handling, and caching, making it efficient and scalable for a variety of use cases

## Usage
### Manual
```sh
# Dependencies (Optional)
nix develop

cargo run --release
# or if you want to build the exec
cargo build --release

# You can find the exec in the <project_dir>/target/release/scrounch_backend
```

### Nix
```
nix build
```

### Docker

```sh
docker build -t <your-image-name> .

docker run <your-image-name>
```

> [!TIP]
> You can also look at [DockerHub](https://hub.docker.com/r/floride/scrounch_backend) for the official image:
> | tag | Explanation |
> |----------|-----------------------------------------|
> | master | The master branch of github (not safe) |
> | latest | The latest safest version (recommended) |
> | vX.X.X | The image version |
