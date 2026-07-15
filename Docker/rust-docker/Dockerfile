FROM debian:bookworm-slim

ARG USERNAME
ARG UID
ARG GID

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        vim \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd -g ${GID} ${USERNAME} && \
    useradd -m -u ${UID} -g ${GID} -s /bin/bash ${USERNAME}

COPY papr /usr/local/bin/

USER ${USERNAME}
WORKDIR /home/${USERNAME}
