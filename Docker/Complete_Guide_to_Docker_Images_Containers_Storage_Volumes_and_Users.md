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

  ------------------------------------------------------------------------
  Action                     Survives?                    Why?
  -------------------------- ---------------------------- ----------------
  `touch /workspace/a.txt`   ✅                           `/workspace` is
                                                          your real
                                                          project
                                                          directory.

  `touch /tmp/b.txt`         ❌                           `/tmp` is inside
                                                          the temporary
                                                          container.

  `cargo install just`       ❌                           Installed inside
                                                          the container
                                                          filesystem.

  `apt install vim`          ❌                           Installed inside
                                                          the container
                                                          filesystem.

  `rust:latest` image        ✅                           Images are
                                                          stored
                                                          independently.

  Your project files         ✅                           They are on your
                                                          host machine.
  ------------------------------------------------------------------------

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

  -----------------------------------------------------------------------
  Component    Purpose          Persists after container deletion?
  ------------ ---------------- -----------------------------------------
  Image        Template used to ✅ Yes
               create           
               containers       

  Container    Running instance ❌ No
               of an image      

  Bind Mount   Shares a host    ✅ Yes (host owns the files)
               directory with   
               the container    

  Named Volume Docker-managed   ✅ Yes
               persistent       
               storage          
  -----------------------------------------------------------------------

Whenever you're unsure whether something will remain after the container
exits, ask:

> **Where is this data being written?**

-   If it's inside the container's own filesystem, it disappears with
    the container.
-   If it's inside a bind mount, it lives on your host.
-   If it's inside a named volume, Docker keeps it for future
    containers.

------------------------------------------------------------------------

# Appendix: Understanding `--user "$(id -u):$(id -g)"` in Docker

Consider the command:

``` bash
docker run \
    --user "$(id -u):$(id -g)" \
    -v "$PWD":/app \
    -w /app \
    rust \
    cargo build --release
```

One of the most useful options here is:

``` bash
--user "$(id -u):$(id -g)"
```

Although it looks cryptic at first, it solves a very common problem when
using Docker for development.

------------------------------------------------------------------------

# The Problem

By default, most Docker containers run as the **root** user.

Inside the container:

``` bash
whoami
```

prints

``` text
root
```

This becomes important when your project directory is shared with the
container:

``` bash
-v "$PWD":/app
```

Because `/app` is a **bind mount**, it is actually your host project
directory.

    Host
    ~/project
          │
          ▼
    Container
    /app

When the container writes files into `/app`, it is really writing files
into your host directory.

------------------------------------------------------------------------

# What Happens Without `--user`

Suppose you build a Rust project:

``` bash
cargo build --release
```

Cargo creates

    target/

Since the container is running as **root**, every generated file is
owned by **root** on the host.

After the build:

``` bash
ls -l
```

might show

``` text
drwxr-xr-x root root target/
```

Now your normal user may not be able to edit, delete, or clean these
files without elevated privileges.

You may eventually need

``` bash
sudo rm -rf target
```

or

``` bash
sudo chown -R $USER:$USER target
```

This is one of the most common frustrations when developing with Docker.

------------------------------------------------------------------------

# What `id -u` and `id -g` Mean

On Linux, every user has

-   a **User ID (UID)**
-   a **Group ID (GID)**

You can see yours:

``` bash
id
```

Example:

``` text
uid=1000(qubit) gid=1000(qubit)
```

To print only the user ID:

``` bash
id -u
```

Example:

``` text
1000
```

To print only the group ID:

``` bash
id -g
```

Example:

``` text
1000
```

------------------------------------------------------------------------

# Command Substitution

The syntax

``` bash
$(...)
```

means

> Run the command and substitute its output.

Example:

``` bash
echo $(id -u)
```

might become

``` bash
echo 1000
```

Similarly,

``` bash
"$(id -u):$(id -g)"
```

becomes

``` text
1000:1000
```

before Docker even starts.

------------------------------------------------------------------------

# What Docker Receives

So

``` bash
--user "$(id -u):$(id -g)"
```

is effectively the same as

``` bash
--user 1000:1000
```

for a typical Linux desktop.

Docker then starts the process using **your UID and GID** instead of
root.

------------------------------------------------------------------------

# What Changes?

Without the option:

    Container process
            │
            ▼
          root
            │
            ▼
    Creates files owned by root

With the option:

    Container process
            │
            ▼
       Your Linux user
     (same UID and GID)
            │
            ▼
    Creates files owned by you

The build output is now owned by your user.

    drwxr-xr-x qubit qubit target/

instead of

    drwxr-xr-x root root target/

------------------------------------------------------------------------

# Why This Is Important

For development, this has several benefits:

-   No root-owned files appear in your project.
-   No need to run `sudo` just to remove build artifacts.
-   Editors can modify generated files normally.
-   File ownership stays consistent between host and container.
-   The workflow feels the same as building directly on the host.

------------------------------------------------------------------------

# When Should You Use It?

It is highly recommended whenever

-   you bind mount your source code,
-   the container writes files back to your project,
-   you compile software inside Docker,
-   or you use Docker as a development environment.

Examples include:

-   Rust (`cargo build`)
-   C/C++ (`cmake`, `make`)
-   Python (generated caches or wheels)
-   Node.js (`npm install`)
-   Go (`go build`)

------------------------------------------------------------------------

# When Might You Not Use It?

Some containers are intentionally designed to run as root because they:

-   install system packages,
-   modify system directories,
-   manage services,
-   or expect root privileges.

In those situations, overriding the user may not be appropriate.

------------------------------------------------------------------------

# Summary

``` bash
--user "$(id -u):$(id -g)"
```

tells Docker:

> "Run the process inside the container using my Linux user ID and group
> ID instead of the default root user."

This small option prevents one of the most common Docker development
issues: **root-owned files appearing in your project after the container
finishes.**

If your workflow involves mounting your project into a container and
generating files there, using `--user "$(id -u):$(id -g)"` is generally
considered a best practice.
