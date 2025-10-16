FROM archlinux:base-devel@sha256:707bb974ce868085034e8d4ba52a2288ee9f152ea47524bad93e7a954268e2db
RUN pacman -Syu --noconfirm \
    && pacman -S --noconfirm \
    appmenu-gtk-module \
    cargo \
    nodejs \
    pnpm \
    coreutils \
    desktop-file-utils \
    gdk-pixbuf2 \
    git \
    glib2 \
    gtk3 \
    jq \
    libappindicator-gtk3 \
    librsvg \
    openssh \
    rustup \
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
