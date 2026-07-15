# Docker Container Data Persistence

When you run a Docker container **without** the `--rm` flag, Docker
preserves the container's writable filesystem after the container exits.

For example:

``` bash
docker run -it     -w /home/$(whoami)/     --name papr-test     papr     bash
```

If you download PDFs or create other files inside the container and then
exit it, those files remain in the container. You can restart the same
container later:

``` bash
docker start -ai papr-test
```

or

``` bash
docker start papr-test
docker exec -it papr-test bash
```

The downloaded PDFs and any other changes you made inside the container
will still be present.

## What happens with `--rm`?

If you instead run:

``` bash
docker run --rm -it ...
```

Docker automatically removes the container as soon as it exits. Since
the container itself is deleted, all files that were stored only inside
its writable filesystem are permanently lost.

## Where is the data stored?

When you do **not** mount a host directory or Docker volume, all files
created inside the container are stored in the container's **writable
layer**.

On Linux, Docker keeps this data under its storage directory, which is
typically:

``` text
/var/lib/docker/
```

If the default `overlay2` storage driver is being used (the most common
case), the writable filesystem for each container is stored somewhere
under:

``` text
/var/lib/docker/overlay2/
```

The exact directory name is managed internally by Docker and should not
be accessed or modified directly.

## Image vs. Container

It is important to distinguish between a Docker **image** and a Docker
**container**:

-   A **Docker image** is a read-only template.
-   A **Docker container** is a running (or stopped) instance of that
    image with its own writable filesystem.

Every `docker run` creates a new container from the image.

-   Restarting the **same container** preserves your downloaded PDFs and
    other files.
-   Creating a **new container** from the same image starts with a fresh
    writable filesystem and will not contain the files from another
    container.

## Persisting Data Outside the Container

The recommended way to store important data, such as downloaded PDFs, is
to mount a host directory or Docker volume. This keeps the files
independent of the container so they remain available even if the
container is deleted.
