FROM rust:1.78.0-bookworm


ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    TZ=Europe/Moscow \
    DEBIAN_FRONTEND=noninteractive

RUN apt-get update -y && \
    apt-get install -y tzdata && \
    ln -fs /usr/share/zoneinfo/Europe/Moscow /etc/localtime && \
    dpkg-reconfigure -f noninteractive tzdata && \
    rm -rf /var/lib/apt/lists/*
    
# ENV WORKSPACE_PATH=/app
    

# WORKDIR "${WORKSPACE_PATH}"