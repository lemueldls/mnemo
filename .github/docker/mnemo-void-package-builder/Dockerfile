FROM ghcr.io/void-linux/void-glibc@sha256:b9228841b368891b56ff516a620b6aa59248811064e164189e2fe543b701e611
RUN xbps-install -Syu || xbps-install -yu xbps \
    && xbps-install -yu \
    && xbps-install -y bash git rustup base-devel curl jq util-linux coreutils binutils bsdtar findutils \
    && xbps-remove -Ooy
RUN rustup-init -y -q \
    && . $HOME/.cargo/env \
    && rustup target add wasm32-unknown-unknown \
    && rm -rf $HOME/.cargo/registry $HOME/.cargo/git
LABEL org.opencontainers.image.source="https://github.com/lemueldls/mnemo"
LABEL org.opencontainers.image.title="Mnemo Void Package Builder"
LABEL org.opencontainers.image.base.name="ghcr.io/void-linux/void-glibc"
