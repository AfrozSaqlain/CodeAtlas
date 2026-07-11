# Understanding Docker Storage: Images, Containers, Bind Mounts, and Volumes

This guide explains what happens when you run a command like:

``` bash
docker run --rm -it \
    -v "$PWD":/workspace \
    -w /workspace \
    rust:latest \
    bash
```

The goal is to understand **what persists**, **what disappears**, and
**why**.

------------------------------------------------------------------------

# A Simple Mental Model

Think of Docker as having four different kinds of storage.

                    Docker Image
                         │
                         ▼
                 Docker Container
                (temporary changes)
                 /              \
                /                \
        Bind Mount           Named Volume
       (your files)      (Docker-managed files)

Before asking "Will this file still exist later?", ask:

> **Where is this data being written?**

That one question answers almost everything.

------------------------------------------------------------------------

# 1. Docker Images

An **image** is a reusable template.

For example,

``` bash
docker run rust:latest
```

uses the image

    rust:latest

An image already contains an operating system and any software that was
bundled with it.

For the official Rust image, this includes:

-   Debian (or another Linux base)
-   `rustc`
-   `cargo`
-   Standard Rust libraries

An image is **read-only**. Docker never modifies it while a container is
running.

Think of it like a blueprint for creating new containers.

                    rust:latest
                 (Docker Image)
                        │
         ┌──────────────┼──────────────┐
         ▼              ▼              ▼
     Container A   Container B   Container C

One image can create many containers.

The image stays on your computer until you explicitly remove it.

List images:

``` bash
docker image ls
```

Remove one:

``` bash
docker image rm rust:latest
```

------------------------------------------------------------------------

# 2. Containers

A **container** is a running instance of an image.

When you execute

``` bash
docker run rust:latest
```

Docker

1.  Starts from the image.
2.  Adds a **temporary writable layer** on top.
3.  Runs the requested program.

```{=html}
<!-- -->
```
                 rust:latest (read-only)
                         │
                         ▼
            +-------------------------+
            | Temporary writable      |
            | layer                   |
            |                         |
            | apt install vim         |
            | cargo install just      |
            | touch hello.txt         |
            +-------------------------+

Everything you change is stored in this writable layer.

If the container is deleted, this layer disappears.

------------------------------------------------------------------------

# 3. What `--rm` Really Does

The option

``` bash
--rm
```

means:

> Delete the **container** automatically after it exits.

Suppose you run

``` bash
docker run --rm -it rust:latest bash
```

Inside the container:

``` bash
apt install vim
cargo install just
touch /tmp/example.txt
```

Everything above is stored in the container's writable layer.

After

``` bash
exit
```

Docker deletes the container.

So:

-   `vim` disappears
-   `just` disappears
-   `/tmp/example.txt` disappears

The **image** remains on your computer.

------------------------------------------------------------------------

# 4. Bind Mounts

Your command contains

``` bash
-v "$PWD":/workspace
```

This is called a **bind mount**.

Suppose your current directory is

    /home/qubit/Templates/Papr

Docker creates this mapping:

    Host
    /home/qubit/Templates/Papr
              │
              ▼
    Container
    /workspace

Nothing is copied.

Both the host and the container are looking at the **same directory**.

If inside the container you create

``` bash
touch /workspace/test.txt
```

your host immediately contains

    ~/Templates/Papr
    ├── Cargo.toml
    ├── src/
    └── test.txt

Similarly,

if you edit `src/main.rs` on your host,

the container immediately sees the updated file.

This is why bind mounts are excellent for development.

------------------------------------------------------------------------

# 5. Named Volumes

A **named volume** is different.

Instead of using one of your folders,

Docker creates storage that it manages.

Example:

``` bash
docker run \
    -v cargo-home:/usr/local/cargo \
    rust:latest
```

Now

    Container
    /usr/local/cargo
              │
              ▼
    Docker Volume
    cargo-home

Unlike a bind mount:

-   You don't choose the folder.
-   Docker stores it internally.
-   It survives even after the container is deleted.

List volumes:

``` bash
docker volume ls
```

Remove one:

``` bash
docker volume rm cargo-home
```

------------------------------------------------------------------------

# 6. Why `cargo install` Disappears

Suppose you run

``` bash
docker run --rm -it \
    -v "$PWD":/workspace \
    rust:latest \
    bash
```

Inside:

``` bash
cargo install just
```

Cargo installs binaries under a directory such as

    /usr/local/cargo

That directory is **inside the container**.

It is **not mounted**.

So after

``` bash
exit
```

the container is deleted.

Therefore

    cargo install just

is gone.

------------------------------------------------------------------------

# 7. Making Cargo Installations Persistent

Now run

``` bash
docker run --rm -it \
    -v "$PWD":/workspace \
    -v cargo-home:/usr/local/cargo \
    rust:latest \
    bash
```

Notice the extra mount:

``` bash
-v cargo-home:/usr/local/cargo
```

Now Cargo writes into the named volume instead of the temporary
container filesystem.

                    Docker Volume
                     cargo-home
                         ▲
                         │
    Container
    /usr/local/cargo

Run

``` bash
cargo install just
```

Exit the container.

Start another container with the **same volume**.

`just` is still installed because it was saved in the volume.

------------------------------------------------------------------------

# 8. Breaking Down Your Command

``` bash
docker run --rm -it \
    -v "$PWD":/workspace \
    -w /workspace \
    rust:latest \
    bash
```

### `docker run`

Create and start a new container.

### `--rm`

Delete the container after it exits.

### `-it`

Create an interactive terminal.

-   `-i` keeps keyboard input open.
-   `-t` allocates a terminal (TTY).

### `-v "$PWD":/workspace`

Share your current directory with the container.

### `-w /workspace`

Start inside `/workspace` instead of `/`.

### `rust:latest`

Use the official Rust image.

### `bash`

Run an interactive Bash shell.

------------------------------------------------------------------------

# 9. What Survives?

Suppose inside the container you run

``` bash
touch /workspace/a.txt
touch /tmp/b.txt

cargo install just

apt update
apt install vim
```

Then you exit.

  --------------------------------------------------------------------------
  Action                     Survives?                      Why?
  -------------------------- ------------------------------ ----------------
  `touch /workspace/a.txt`   ✅                             `/workspace` is
                                                            your real
                                                            project
                                                            directory.

  `touch /tmp/b.txt`         ❌                             `/tmp` is inside
                                                            the temporary
                                                            container.

  `cargo install just`       ❌                             Installed inside
                                                            the container
                                                            filesystem.

  `apt install vim`          ❌                             Installed inside
                                                            the container
                                                            filesystem.

  `rust:latest` image        ✅                             Images are
                                                            stored
                                                            independently.

  Your project files         ✅                             They are on your
                                                            host machine.
  --------------------------------------------------------------------------

If you instead mount a named volume for Cargo:

``` bash
-v cargo-home:/usr/local/cargo
```

then:

  Action                 Survives?
  ---------------------- -----------
  `cargo install just`   ✅

------------------------------------------------------------------------

# 10. Summary

Docker has four important concepts:

  -----------------------------------------------------------------------------
  Component     Purpose          Persists after container deletion?
  ------------- ---------------- ----------------------------------------------
  Image         Template used to ✅ Yes
                create           
                containers       

  Container     Running instance ❌ No
                of an image      

  Bind Mount    Shares a host    ✅ Yes (host owns the files)
                directory with   
                the container    

  Named Volume  Docker-managed   ✅ Yes
                persistent       
                storage          
  -----------------------------------------------------------------------------

Whenever you're unsure whether something will remain after the container
exits, ask:

> **Where is this data being written?**

-   If it's inside the container's own filesystem, it disappears with
    the container.
-   If it's inside a bind mount, it lives on your host.
-   If it's inside a named volume, Docker keeps it for future
    containers.
