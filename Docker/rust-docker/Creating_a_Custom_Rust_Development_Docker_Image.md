# Creating a Custom Rust Development Docker Image

This guide explains how to create a custom Docker image for Rust
development that runs as your normal Linux user instead of `root`.

Unlike using:

``` bash
docker run --user "$(id -u):$(id -g)" ...
```

this approach creates an actual user inside the image, so tools like
`whoami`, the shell prompt, and `$HOME` work as expected.

------------------------------------------------------------------------

# Why Create a Custom Image?

When you use the official image directly:

``` bash
docker run --rm -it \
    --user "$(id -u):$(id -g)" \
    rust:latest \
    bash
```

the process runs with your numeric UID and GID, but the image has no
matching entry in `/etc/passwd`.

As a result you may see:

``` text
I have no name!@container:/workspace$
```

The files created on bind mounts are still owned by your user on the
host, but the container does not know your username.

Creating a custom image fixes this by adding a real user.

------------------------------------------------------------------------

# Before We Begin: Image vs Container

A common source of confusion is the difference between an **image** and
a **container**.

    Dockerfile
         │
         ▼
    docker build
         │
         ▼
    Docker Image
         │
         ▼
    docker run
         │
         ▼
    Docker Container

-   A **Dockerfile** is a recipe.
-   `docker build` follows the recipe and creates an **image**.
-   `docker run` creates a **container** from that image.

You can create many containers from the same image without rebuilding
it.

Think of it like baking:

    Recipe
       │
       ▼
    Bake
       │
       ▼
    Cake
       │
       ▼
    Serve slices

The recipe is the Dockerfile, the cake is the image, and each slice is a
container.

------------------------------------------------------------------------

# Dockerfile

``` dockerfile
FROM rust:latest

ARG USERNAME
ARG UID
ARG GID

RUN groupadd -g ${GID} ${USERNAME} \
 && useradd -m -u ${UID} -g ${GID} -s /bin/bash ${USERNAME}

USER ${USERNAME}
WORKDIR /home/${USERNAME}
```

------------------------------------------------------------------------

# Understanding Every Instruction

## `FROM`

Selects the base image.

``` dockerfile
FROM rust:latest
```

Everything in this image becomes available in your own image.

------------------------------------------------------------------------

## `ARG`

``` dockerfile
ARG USERNAME
ARG UID
ARG GID
```

`ARG` defines **build-time variables**.

They only exist while Docker is building the image.

Later we provide them with

``` bash
--build-arg USERNAME=$(whoami)
--build-arg UID=$(id -u)
--build-arg GID=$(id -g)
```

------------------------------------------------------------------------

## `RUN`

``` dockerfile
RUN groupadd -g ${GID} ${USERNAME} \
 && useradd -m -u ${UID} -g ${GID} -s /bin/bash ${USERNAME}
```

`RUN` executes commands **during the image build**.

The results become part of the final image.

### `groupadd`

Creates a Linux group.

Example:

``` bash
groupadd -g 1000 qubit
```

### `useradd`

Creates a Linux user.

``` bash
useradd -m -u 1000 -g 1000 -s /bin/bash qubit
```

  Option   Meaning
  -------- ----------------------------
  `-m`     Create the home directory.
  `-u`     Set the UID.
  `-g`     Set the primary group.
  `-s`     Set the login shell.

------------------------------------------------------------------------

## `USER`

Changes the default runtime user.

Without this instruction the container starts as `root`.

------------------------------------------------------------------------

## `WORKDIR`

Sets the default working directory whenever a container starts from this
image.

------------------------------------------------------------------------

# Building the Image

``` bash
docker build \
    --build-arg USERNAME=$(whoami) \
    --build-arg UID=$(id -u) \
    --build-arg GID=$(id -g) \
    -t papr-rust .
```

## What Does `docker build` Do?

It **creates an image**, not a container.

Step by step:

1.  Docker reads the `Dockerfile` from the current directory (`.`).
2.  The current directory becomes the **build context** (excluding files
    ignored by `.dockerignore`).
3.  Docker executes each instruction one by one.
4.  Every instruction creates a new immutable image layer.
5.  Docker combines those layers into a new image called `papr-rust`.

```{=html}
<!-- -->
```
    Layer 5  WORKDIR
    ────────────────────
    Layer 4  USER
    ────────────────────
    Layer 3  RUN useradd
    ────────────────────
    Layer 2  ARG
    ────────────────────
    Layer 1  rust:latest

After the build finishes:

``` bash
docker image ls
```

shows something like

``` text
REPOSITORY   TAG      IMAGE ID
papr-rust    latest   abc123...
```

Notice that **no container has been created yet**.

------------------------------------------------------------------------

## Understanding the Build Arguments

The shell expands:

``` bash
$(whoami)
```

to your username.

Likewise,

``` bash
$(id -u)
```

becomes your UID, and

``` bash
$(id -g)
```

becomes your GID.

Docker effectively receives:

``` bash
docker build \
    --build-arg USERNAME=qubit \
    --build-arg UID=1000 \
    --build-arg GID=1000 \
    -t papr-rust .
```

------------------------------------------------------------------------

# Running a Container

Once the image exists, create a container from it:

``` bash
docker run --rm -it \
    -v "$PWD":/home/$(whoami)/papr \
    -w /home/$(whoami)/papr \
    papr-rust \
    bash
```

Notice that `--user` is no longer needed because the image already
contains your user.

------------------------------------------------------------------------

# What You Gain

-   Proper username in the prompt.
-   `whoami` works.
-   Correct `$HOME`.
-   Files created on bind mounts belong to your host user.
-   No `I have no name!`.

------------------------------------------------------------------------

# Things to Keep in Mind

1.  UID and GID are baked into the image at build time. Rebuild if they
    change.
2.  This is excellent for personal development. Other users should build
    their own copy with their own UID/GID.
3.  Updating the Dockerfile requires rebuilding the image.
4.  This affects **who owns files**, not **where they are stored**. Bind
    mounts and named volumes still control persistence.

------------------------------------------------------------------------

# Summary

  Instruction / Command   Purpose
  ----------------------- --------------------------------------------
  `FROM`                  Base image
  `ARG`                   Build-time variable
  `RUN`                   Execute commands while building the image
  `groupadd`              Create Linux group
  `useradd`               Create Linux user
  `USER`                  Default runtime user
  `WORKDIR`               Default working directory
  `docker build`          Create an image
  `docker run`            Create and start a container from an image
