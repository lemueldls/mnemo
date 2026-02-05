FROM archlinux:base-devel@sha256:938749229e882c7a5b60a1de200b4b6c139264595656e594ccb07f53b944f7cf

RUN pacman -Syu --noconfirm \
    && pacman -S --noconfirm --needed \
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
    webkit2gtk-4.1 \
    sudo \
    wget \
    libsoup \
    && useradd -m -G wheel -s /bin/bash runner \
    && echo '%wheel ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers \
    && pacman -Scc --noconfirm

LABEL org.opencontainers.image.source="https://github.com/lemueldls/mnemo"
LABEL org.opencontainers.image.title="Mnemo Arch AUR Builder"
LABEL org.opencontainers.image.base.name="archlinux:base-devel"
