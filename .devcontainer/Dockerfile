FROM ubuntu:mantic

ENV DEBIAN_FRONTEND=noninteractive
ENV SHELL /bin/bash
SHELL ["/bin/bash", "-c"]

# fix /tmp permissions
RUN chmod 1777 /tmp
WORKDIR /tmp

RUN apt-get update && apt-get upgrade -q -y && \
    apt-get install -q -y --no-install-recommends \
    openssh-client \
    curl \
    wget \
    unzip \
    gnupg \
    gnupg2 \
    git \
    git-lfs \
    coreutils \
    lsb-release \
    ca-certificates \
    pkg-config \
    apt-utils \
    apt-transport-https \
    software-properties-common \
    dirmngr \
    locales \
    tzdata \
    build-essential \
    bash-completion \
    zstd \
    libzstd-dev \
    llvm-16 \
    llvm-16-dev \
    llvm-16-runtime \
    llvm-16-linker-tools \
    libpolly-16-dev \
    python3 \
    python3-apt \
    python3-pip \
    python3-dev \
    python3-venv \
    libpython3-dev \
    && rm -rf /var/lib/apt/lists/* && apt-get clean

# setup locale
RUN locale-gen en_US en_US.UTF-8 && update-locale LC_ALL=en_US.UTF-8 LANG=en_US.UTF-8
ENV LANG=en_US.UTF-8
ENV LC_ALL=C.UTF-8
ENV PYTHONIOENCODING=utf-8

ENV PYTHONPATH="/workspaces/cmput651project/rust/target/release:/workspaces/cmput651project/rust/target/debug"

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

# setup better up/down arrow history search
COPY .inputrc $HOME/.inputrc

# link .bash_history to save history
RUN ln -s /workspaces/cmput651project/.bash_history $HOME/.bash_history

CMD ["bash"]
