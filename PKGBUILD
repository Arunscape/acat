# Maintainer: Arun Woosaree <arun@woosaree.xyz>
pkgname=acat
pkgver=0.1.1.20.ga1af9f7
pkgrel=1
makedepends=('cargo' 'git')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="Like lolcat, but more chaotic. Not feature-complete yet."
license=('GPL-3.0-or-later')
url="https://github.com/Arunscape/acat"
provides=("acat")
source=("$pkgname::git+https://github.com/Arunscape/acat")
sha1sums=('SKIP')

pkgver() {
    cd "$pkgname"
    echo "$(grep '^version =' Cargo.toml|head -n1|cut -d\" -f2).$(git rev-list --count HEAD).g$(git rev-parse --short HEAD)" | tr '-' '.'
}

build() {
    cd $pkgname
    cargo build --release --locked
}
    
check() {
    cd $pkgname
    cargo test --release --locked
}

package() {
  cd "$srcdir/$pkgname"
  install -Dm755 "target/release/acat" "${pkgdir}/usr/bin/acat"
}
