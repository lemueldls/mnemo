#!/usr/bin/env bash

version=$1
sha256sum=$2

if [[ -z "$version" || -z "$sha256sum" ]]; then
    echo "Usage: $0 <version> <sha256sum>" >&2
    exit 1
fi

cat <<EOF
pkgname=mnemo
version=$version
revision=1
archs="x86_64"
build_style=tauri
hostmakedepends="desktop-file-utils"
makedepends="nodejs pnpm libwebkit2gtk41-devel wget file gtk+3-devel librsvg-devel"
depends="libwebkit2gtk41"
short_desc="Local-first note-taking app leveraging the Typst ecosystem."
maintainer="Orphaned <orphan@voidlinux.org>"
license="AGPL-3.0"
homepage="https://github.com/lemueldls/mnemo"
distfiles="https://github.com/lemueldls/mnemo/archive/refs/tags/\${pkgname}-v\${version}.tar.gz"
checksum=$sha256sum
_builddir="\$pkgname-\$pkgname-v\$pkgver/platform"

do_build() {
	ln -s /host/.cargo /tmp
	ln -s /host/.rustup /tmp
	. /tmp/.cargo/env
	cargo fetch --locked --target "\$(rustc -vV | sed -n 's/host: //p')"
  export NODE_OPTIONS=--max-old-space-size=8192
  export NUXT_PUBLIC_API_BASE_URL="https://mnemo.world"
  pnpm tauri build -b deb -c "platform/tauri/tauri.package.conf.json"
}

do_install() {
	vcopy platform/target/release/bundle/deb/Mnemo_\${version}_amd64/data/usr /
	vlicense LICENSE
}
EOF