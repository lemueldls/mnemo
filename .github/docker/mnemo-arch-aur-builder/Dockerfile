FROM archlinux:base-devel@sha256:938749229e882c7a5b60a1de200b4b6c139264595656e594ccb07f53b944f7cf

RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm --needed \
    appmenu-gtk-module \
    cargo \
    coreutils \
    desktop-file-utils \
    gdk-pixbuf2 \
    git \
    glib2 \
    gtk3 \
    jq \
    libappindicator-gtk3 \
    librsvg \
    namcap \
    openssh \
    rust \
    rust-wasm \
    webkit2gtk-4.1 \
    sudo \
    wget \
    libsoup3
RUN useradd -m -G wheel -s /bin/bash runner
RUN echo '%wheel ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers
RUN pacman -Scc --noconfirm

LABEL org.opencontainers.image.source="https://github.com/lemueldls/mnemo"
LABEL org.opencontainers.image.title="Mnemo Arch AUR Builder"
LABEL org.opencontainers.image.base.name="archlinux:base-devel"
