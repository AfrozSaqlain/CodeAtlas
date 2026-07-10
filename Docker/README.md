# 🐳 Docker Quick Reference Guide

![Docker](https://img.shields.io/badge/Docker-Containerization-blue?logo=docker)
![Linux](https://img.shields.io/badge/Platform-Linux-black?logo=linux)
![License](https://img.shields.io/badge/License-MIT-green)

A minimal and practical Docker cheat sheet for building, running, and managing containers.

---

# 🚀 Docker Service

### Enable Docker on boot

```bash
sudo systemctl enable docker
```

### Disable Docker on boot

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

# 📦 Containers

### Running containers

```bash
docker ps
```

### All containers

```bash
docker ps -a
```

### Run interactively

```bash
docker run -it ubuntu bash
```

### Run and remove after exit

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

### Remove a container

```bash
docker rm <container>
```

---

# 🖼️ Images

### List images

```bash
docker image ls
```

### Pull an image

```bash
docker pull ubuntu
```

### Remove an image

```bash
docker rmi <image>
```

### Inspect image history

```bash
docker history <image>
```

---

# 💾 Volumes

Persist data outside the container.

```bash
docker run -it \
    -v ~/mydata:/workspace \
    ubuntu bash
```

Files inside `~/mydata` remain even after the container is removed.

---

# 🧹 Cleanup

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

### Remove everything unused

```bash
docker system prune
```

### Check Docker disk usage

```bash
docker system df
```

---

# ⚡ Buildx (Recommended)

Docker recommends using **Buildx** (BuildKit) instead of the legacy builder.

### Build an image

```bash
docker buildx build \
    --load \
    -t my-image:latest .
```

`--load` imports the image into your local Docker image store.

### List builders

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

# 🐍 Dockerizing a Python CLI

## Project Structure

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

```text
.git
.github
__pycache__
*.pyc
*.egg-info
build
dist
.venv
venv
```

## Build

```bash
docker buildx build \
    --load \
    -t my-cli-app:latest .
```

## Run

```bash
docker run --rm -it my-cli-app:latest
```

## Tag

```bash
docker tag my-cli-app:latest username/my-cli-app:latest
```

## Push

```bash
docker push username/my-cli-app:latest
```

---

# 📊 Useful Commands

See Docker disk usage:

```bash
docker system df
```

See image layers:

```bash
docker history <image>
```

See BuildKit builders:

```bash
docker buildx ls
```

---

# 🚀 Typical Workflow

```bash
# Edit your code

docker buildx build --load -t my-cli-app:latest .

docker run --rm -it my-cli-app:latest

docker push username/my-cli-app:latest
```

---

Happy Dockering! 🐳
