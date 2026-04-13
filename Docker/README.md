# 🐳 Docker Quick Reference Guide

![Docker](https://img.shields.io/badge/Docker-Containerization-blue?logo=docker)
![Linux](https://img.shields.io/badge/Platform-Linux-black?logo=linux)
![License](https://img.shields.io/badge/License-MIT-green)

A minimal, practical, and clean guide to working with Docker containers
and images.\
Perfect for beginners and daily use.

------------------------------------------------------------------------

## 🚀 Service Management

### Enable Docker at boot

``` bash
sudo systemctl enable docker
```

### Disable Docker at boot

``` bash
sudo systemctl disable docker
```

### Start Docker

``` bash
sudo systemctl start docker
```

### Stop Docker

``` bash
sudo systemctl stop docker
```

### Check status

``` bash
systemctl status docker
```

------------------------------------------------------------------------

## 📦 Containers

### List running containers

``` bash
docker ps
```

### List all containers (including stopped)

``` bash
docker ps -a
```

### Run a container (interactive shell)

``` bash
docker run -it ubuntu bash
```

### Run and auto-remove container (clean usage)

``` bash
docker run --rm hello-world
```

### Start an existing container

``` bash
docker start -i <container_id>
```

### View container logs

``` bash
docker logs <container_id>
```

### Inspect container (detailed info)

``` bash
docker inspect <container_id>
```

------------------------------------------------------------------------

## 🖼️ Images

### List images

``` bash
docker image ls
```

### Pull image

``` bash
docker pull ubuntu
```

### Remove image

``` bash
docker rmi <image_id>
```

------------------------------------------------------------------------

## 💾 Persistence (Important)

### Use volumes (recommended for real work)

``` bash
docker run -it -v ~/mydata:/home/ubuntu ubuntu bash
```

✔️ Files persist even after container exits

------------------------------------------------------------------------

## 🧹 Cleanup

### Remove stopped containers

``` bash
docker container prune
```

### Remove unused images

``` bash
docker image prune
```

### Remove everything unused

``` bash
docker system prune
```

------------------------------------------------------------------------

## 🐍 Dockerizing a Python CLI Application

### Project Structure

``` bash
project-root/
├── pyproject.toml
├── Dockerfile
└── src/
    └── my_package/
        ├── __init__.py
        └── cli.py
```

### Key Points

-   Use `src/` layout
-   Define CLI entry point in `pyproject.toml`
-   Add dependencies
-   Avoid GUI (use headless mode)

### Dockerfile

``` dockerfile
FROM python:3.11-slim

ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1

RUN apt-get update && apt-get install -y \
    libglib2.0-0 \
    libsm6 \
    libxext6 \
    libxrender1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt/app

COPY pyproject.toml .
COPY src/ ./src/

RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir .

CMD ["bash", "-lc", "cd ~ && exec bash"]
```

### Build

``` bash
docker build -t my-cli-app .
```

### Run

``` bash
docker run -it --rm my-cli-app
```

### Tag

``` bash
docker tag my-cli-app <your-username>/my-cli-app:latest
```

### Push

``` bash
docker push <your-username>/my-cli-app:latest
```

------------------------------------------------------------------------

## 🏁 You're Ready!

Happy Dockering 🚀
