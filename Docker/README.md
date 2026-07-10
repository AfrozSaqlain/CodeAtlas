# 🐳 Docker Quick Reference Guide

![Docker](https://img.shields.io/badge/Docker-Containerization-blue?logo=docker)
![Linux](https://img.shields.io/badge/Platform-Linux-black?logo=linux)
![License](https://img.shields.io/badge/License-MIT-green)

A concise yet practical guide to Docker for everyday development. This document covers installing Docker, setting up Buildx, building images, running containers, cleaning up resources, and packaging Python CLI applications.

---

# 🚀 Installing Docker on Arch Linux

Docker is available directly from the official Arch repositories.

## Install Docker

```bash
sudo pacman -S docker
```

## Install Buildx (Recommended)

Docker recommends using **Buildx (BuildKit)** instead of the legacy builder.

```bash
sudo pacman -S docker-buildx
```

## Enable and start Docker

```bash
sudo systemctl enable --now docker
```

## Allow Docker to run without `sudo` (Optional)

```bash
sudo usermod -aG docker $USER
```

Log out and log back in for the changes to take effect.

Verify the installation:

```bash
docker version
docker buildx version
```

---

# ⚡ One-Time Buildx Setup

Buildx is Docker's modern image builder. It provides:

- Faster builds
- Better layer caching
- Parallel build execution
- Multi-platform builds
- Replaces the legacy `docker build`

Create a Buildx builder and make it the default:

```bash
docker buildx create --use
```

Initialize (bootstrap) the builder:

```bash
docker buildx inspect --bootstrap
```

Verify that everything is working:

```bash
docker buildx ls
```

> **Note:** These commands are required only once. After the builder is created, all future builds automatically use it.

---

# 🚀 Managing the Docker Service

Docker runs as a background service (daemon).

### Enable Docker at boot

```bash
sudo systemctl enable docker
```

### Disable Docker at boot

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

A **container** is a running (or previously run) instance of a Docker image.

### List running containers

```bash
docker ps
```

### List all containers

```bash
docker ps -a
```

### Run a container interactively

```bash
docker run -it ubuntu bash
```

### Run and automatically remove after exit

```bash
docker run --rm hello-world
```

### Start an existing container

```bash
docker start -ai <container>
```

### View logs

```bash
docker logs <container>
```

### Inspect container

```bash
docker inspect <container>
```

### Remove a stopped container

```bash
docker rm <container>
```

---

# 🖼️ Working with Docker Images

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

### Inspect image layers

Useful for understanding image size and debugging Dockerfiles.

```bash
docker history <image>
```

---

# 💾 Persisting Data with Volumes

Containers are temporary. To persist your data, mount a directory from your host machine.

```bash
docker run -it \
    -v ~/mydata:/workspace \
    ubuntu bash
```

Everything inside `/workspace` remains stored on your host even after the container is removed.

---

# ⚡ Building Images with Buildx

Build Docker images using the modern BuildKit backend.

### Build an image

```bash
docker buildx build \
    --load \
    -t my-image:latest .
```

The `--load` option imports the built image into your local Docker image store.

### List Buildx builders

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

Docker stores images, stopped containers, build cache, and volumes. These commands help reclaim disk space.

### Remove stopped containers

```bash
docker container prune
```

### Remove dangling images

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

### View Docker disk usage

```bash
docker system df
```

---

# 🐍 Dockerizing a Python CLI Application

A recommended project structure:

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

Prevent unnecessary files from being copied into the build context.

```text
.git
.github
.gitignore
__pycache__
*.pyc
*.pyo
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

## Tag the image

```bash
docker tag my-cli-app:latest username/my-cli-app:latest
```

## Push to Docker Hub

```bash
docker push username/my-cli-app:latest
```

---

# 🔍 Useful Debugging Commands

### Docker disk usage

```bash
docker system df
```

### Image layer history

```bash
docker history <image>
```

### Buildx builders

```bash
docker buildx ls
```

### Docker contexts

```bash
docker context ls
```

---

# 🚀 Typical Development Workflow

A typical development cycle looks like this:

```bash
# Edit your source code

docker buildx build --load -t my-cli-app:latest .

docker run --rm -it my-cli-app:latest

docker push username/my-cli-app:latest
```

---

# 📚 Docker Terminology

| Term | Meaning |
|------|---------|
| **Image** | A read-only template used to create containers. |
| **Container** | A running instance of an image. |
| **Dockerfile** | A file containing instructions for building an image. |
| **Volume** | Persistent storage shared between the host and a container. |
| **BuildKit / Buildx** | Docker's modern build system providing faster builds, better caching, and multi-platform support. |

---

# 🎉 Happy Dockering!

Docker becomes much easier once you remember the workflow:

> **Write a Dockerfile → Build an image → Run a container → Push the image (optional).**

Enjoy containerizing your applications! 🐳
