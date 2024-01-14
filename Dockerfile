FROM rust:1.75.0

RUN set -ex; \
      apt-get update; \
      apt-get install -y --no-install-recommends \
        build-essential \
        gdb \
      ; \
      rm -r /var/lib/apt/lists/*
