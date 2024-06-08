FROM rust:1.78.0-bookworm

ARG HOST_USER_NAME="exadra37"
ARG HOST_UID="1000"
ARG HOST_GID="1000"

ENV HOST_USER_NAME=${HOST_USER_NAME} \
    HOST_HOME=/home/${HOST_USER_NAME} \
    HOST_UID=${HOST_UID} \
    HOST_GID=${HOST_GID}

ENV WORKSPACE_PATH=${HOST_HOME}/app/src-tauri

RUN groupadd -g "${HOST_GID}" "${HOST_USER_NAME}" && \
    useradd --create-home --uid "${HOST_UID}" --gid "${HOST_GID}" "${HOST_USER_NAME}"

RUN apt update -y && apt upgrade -y 
RUN apt install -y g++-mingw-w64-x86-64 

RUN rustup target add x86_64-pc-windows-gnu 
RUN rustup toolchain install stable-x86_64-pc-windows-gnu 
RUN apt install -y nsis lld llvm
RUN cargo install create-tauri-app --locked tauri-cli cargo-xwin
RUN apt install -y libwebkit2gtk-4.1-dev \
build-essential \
curl \
wget \
file \
libxdo-dev \
libssl-dev \
libayatana-appindicator3-dev \
librsvg2-dev \
nano \
mc

WORKDIR "${WORKSPACE_PATH}"

RUN apt-get install -y libsoup2.4-dev
RUN apt install -y mingw-w64

# BUILD WITH:
# docker build --build-arg "HOST_USER_NAME=$(id -un)" --build-arg "HOST_UID=$(id -u)" --build-arg "HOST_GID=$(id -g)" . -t tauri/windows

# RUN WIDTH:
# docker run --rm -v $(pwd):/home/$(id -un)/app/src-tauri

# CMD ["cargo", "build", "--release", "--target", "x86_64-pc-windows-gnu"] 