# 🐳 Docker Quick Reference Guide

![Docker](https://img.shields.io/badge/Docker-Containerization-blue?logo=docker)
![Linux](https://img.shields.io/badge/Platform-Linux-black?logo=linux)
![License](https://img.shields.io/badge/License-MIT-green)

A minimal, practical, and clean guide to working with Docker containers and images.  
Perfect for beginners and daily use.

---

## 🚀 Service Management

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

### Check status
```bash
systemctl status docker
```

---

## 📦 Containers

### List running containers
```bash
docker ps
```

### List all containers (including stopped)
```bash
docker ps -a
```

### Run a container (interactive shell)
```bash
docker run -it ubuntu bash
```

### Run and auto-remove container (clean usage)
```bash
docker run --rm hello-world
```

### Start an existing container
```bash
docker start -i <container_id>
```

### View container logs
```bash
docker logs <container_id>
```

### Inspect container (detailed info)
```bash
docker inspect <container_id>
```

---

## 🖼️ Images

### List images
```bash
docker image ls
```

### Pull image
```bash
docker pull ubuntu
```

### Remove image
```bash
docker rmi <image_id>
```

---

## 💾 Persistence (Important)

### Use volumes (recommended for real work)
```bash
docker run -it -v ~/mydata:/home/ubuntu ubuntu bash
```

- `~/mydata` → Host directory  
- `/home/ubuntu` → Container directory  

✔️ Files persist even after container exits

---

## 🧹 Cleanup

### Remove stopped containers
```bash
docker container prune
```

### Remove unused images
```bash
docker image prune
```

### Remove everything unused (⚠️ aggressive)
```bash
docker system prune
```

---

## ⚡ Useful Tips

- Containers are **ephemeral** (temporary by default)
- Use `--rm` to avoid clutter
- Use volumes for **persistent data**
- Each `docker run` creates a **new container**

---

## 🧠 Mental Model

| Concept   | Meaning             |
|-----------|---------------------|
| Image     | Blueprint/template  |
| Container | Running instance    |
| Volume    | Persistent storage  |

---

## 🧪 Test Installation

```bash
docker run hello-world
```

---

## 📌 Optional: Run Docker without sudo

```bash
sudo usermod -aG docker $USER
```

Then log out and log back in.

---

## 🚀 Example Workflow

```bash
# Pull image
docker pull ubuntu

# Run interactive container
docker run -it ubuntu bash

# Exit container
exit

# Restart same container
docker ps -a
docker start -i <container_id>
```

---

## 🏁 You're Ready!

You now know how to:
- Run and manage containers
- Handle images efficiently
- Persist data properly
- Keep your system clean

Happy Dockering 🚀
