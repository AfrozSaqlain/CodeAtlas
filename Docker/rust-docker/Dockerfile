FROM ubuntu:24.04

ARG USERNAME
ARG UID
ARG GID

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        vim \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN if id -u ${UID} >/dev/null 2>&1; then \
        usermod -l ${USERNAME} -d /home/${USERNAME} -m "$(getent passwd ${UID} | cut -d: -f1)"; \
    else \
        if ! getent group ${GID} >/dev/null; then \
            groupadd -g ${GID} ${USERNAME}; \
        fi; \
        useradd -m -u ${UID} -g ${GID} -s /bin/bash ${USERNAME}; \
    fi

COPY papr /usr/local/bin/

USER ${USERNAME}
WORKDIR /home/${USERNAME}
