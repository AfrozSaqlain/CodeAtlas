# 🐳 Docker Quick Reference Guide

![Docker](https://img.shields.io/badge/Docker-Containerization-blue?logo=docker)
![Linux](https://img.shields.io/badge/Platform-Linux-black?logo=linux)
![License](https://img.shields.io/badge/License-MIT-green)

A concise yet practical guide to Docker for everyday development. This covers the most common workflows: managing Docker, building images, running containers, cleaning up resources, and packaging Python applications.

---

# 🚀 Managing the Docker Service

Docker runs as a background service (daemon). These commands control the service itself.

### Enable Docker at system startup

```bash
sudo systemctl enable docker
```

### Disable Docker from starting automatically

```bash
sudo systemctl disable docker
```

### Start Docker

```bash
sudo systemctl start docker
```

### Stop Docker

```bash
sudo systemctl stop docker
```

### Check Docker status

```bash
systemctl status docker
```

---

# 📦 Working with Containers

A **container** is a running (or previously run) instance of an image.

### List running containers

```bash
docker ps
```

### List all containers (including stopped ones)

```bash
docker ps -a
```

### Run a container interactively

```bash
docker run -it ubuntu bash
```

### Run a container and automatically remove it after exit

```bash
docker run --rm hello-world
```

### Start an existing container

```bash
docker start -ai <container>
```

### View container logs

```bash
docker logs <container>
```

### Inspect container details

```bash
docker inspect <container>
```

### Remove a stopped container

```bash
docker rm <container>
```

---

# 🖼️ Working with Images

An **image** is a blueprint used to create containers.

### List downloaded images

```bash
docker image ls
```

### Download an image

```bash
docker pull ubuntu
```

### Remove an image

```bash
docker rmi <image>
```

### View image layers

Useful for understanding image size and debugging Dockerfiles.

```bash
docker history <image>
```

---

# 💾 Persisting Data with Volumes

Containers are temporary. If you want your files to remain after the container is deleted, mount a directory from your host machine.

```bash
docker run -it \
    -v ~/mydata:/workspace \
    ubuntu bash
```

Anything written to `/workspace` inside the container is stored in `~/mydata` on your computer.

---

# ⚡ Building Images with Buildx (Recommended)

Docker now recommends **Buildx (BuildKit)** instead of the legacy `docker build` command.

### Build an image

```bash
docker buildx build \
    --load \
    -t my-image:latest .
```

The `--load` flag imports the built image into your local Docker image store so it behaves just like images built with the legacy builder.

### Show available builders

```bash
docker buildx ls
```

### Remove unused BuildKit cache

```bash
docker buildx prune
```

### Remove all BuildKit cache

```bash
docker buildx prune -a
```

---

# 🧹 Cleaning Up Docker Resources

Docker stores images, stopped containers, caches, and volumes. These commands help reclaim disk space.

### Remove all stopped containers

```bash
docker container prune
```

### Remove dangling (unused) images

```bash
docker image prune
```

### Remove all unused images

```bash
docker image prune -a
```

### Remove all unused Docker resources

```bash
docker system prune
```

### Check Docker disk usage

```bash
docker system df
```

---

# 🐍 Dockerizing a Python CLI Application

A common project layout for Python applications.

```text
project/
├── Dockerfile
├── pyproject.toml
├── .dockerignore
└── src/
    └── my_package/
```

## Example Dockerfile

```dockerfile
FROM python:3.11-slim

ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libglib2.0-0 \
        libsm6 \
        libxext6 \
        libxrender1 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /opt/app

COPY pyproject.toml .
COPY src ./src

RUN pip install --no-cache-dir .

CMD ["bash", "-lc", "cd ~ && exec bash"]
```

## Recommended `.dockerignore`

Prevent unnecessary files from being copied into the Docker build context.

```text
.git
.github
__pycache__
*.pyc
*.egg-info
.ipynb_checkpoints
build
dist
.venv
venv
```

## Build the image

```bash
docker buildx build \
    --load \
    -t my-cli-app:latest .
```

## Run the application

```bash
docker run --rm -it my-cli-app:latest
```

## Tag an image for Docker Hub

```bash
docker tag my-cli-app:latest username/my-cli-app:latest
```

## Push to Docker Hub

```bash
docker push username/my-cli-app:latest
```

---

# 🔍 Useful Commands for Debugging

### View Docker disk usage

```bash
docker system df
```

### Inspect image layers

```bash
docker history <image>
```

### List Buildx builders

```bash
docker buildx ls
```

### List all Docker contexts

```bash
docker context ls
```

---

# 🚀 Typical Development Workflow

A common development cycle looks like this:

```bash
# Edit your source code

docker buildx build --load -t my-cli-app:latest .

docker run --rm -it my-cli-app:latest

docker push username/my-cli-app:latest
```

---

# 💡 Key Concepts

- **Image** → A read-only template used to create containers.
- **Container** → A running instance of an image.
- **Dockerfile** → Instructions for building an image.
- **Volume** → Persistent storage shared between the host and a container.
- **BuildKit / Buildx** → Docker's modern image builder with better caching, performance, and multi-platform support.

---

Happy Dockering! 🐳
