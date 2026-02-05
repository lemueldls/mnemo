FROM ghcr.io/void-linux/void-glibc@sha256:28fdbec439e19df1016cf5d9bb059618d031c710d798fe8bd72716928b221d3b

RUN xbps-install -Syu xbps \
    && xbps-install -Syu \
    && xbps-install -Sy \
    base-devel \
    bash \
    binutils \
    bsdtar \
    coreutils \
    curl \
    findutils \
    git \
    jq \
    rustup \
    util-linux \
    xtools \
    && xbps-remove -Ooy

RUN rustup-init -y -q \
    && . $HOME/.cargo/env \
    && rm -rf $HOME/.cargo/registry $HOME/.cargo/git

LABEL org.opencontainers.image.source="https://github.com/lemueldls/mnemo"
LABEL org.opencontainers.image.title="Mnemo Void Package Builder"
LABEL org.opencontainers.image.base.name="ghcr.io/void-linux/void-glibc"
