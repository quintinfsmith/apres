import os
import toml

def build_pkgbuild(manifest):
    output = """
# Maintainer: %s
pkgname=%s
pkgver=%s
pkgrel=1
epoch=
pkgdesc="%s"
arch=('x86_64')
url=""
license=('GPL')
groups=()
depends=('python>=3.8.0')
makedepends=()
checkdepends=()
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=("$pkgname-$pkgver.tar.gz")
noextract=()
validpgpkeys=()

prepare() {
    cd "$pkgname-$pkgver"
}

build() {
    cd "$pkgname-$pkgver"
}

check() {
    cd "$pkgname-$pkgver"
}

package() {
    cd "$pkgname-$pkgver"
    mv ./usr/ "$pkgdir/"
}
""" % (
    ",".join(manifest['package']['authors']),
    manifest['package']['name'],
    manifest['package']['version'],
    manifest['package']['description']
)

    return output


manifest = toml.load("Cargo.toml")
name = manifest['package']['name']
version = manifest['package']['version']
description = manifest['package']['description']
os.mkdir(name)
os.chdir(name);

folder = "%s-%s" % (name, version)
os.system('cargo build --release')

os.system('rm target/release/build -rf')
os.system("mkdir %s/usr/lib/%s -p" % (folder, name))

os.system("mkdir %s/usr/lib/python3.8/apres/ -p" % folder)
os.system("cp ../src/python_bindings/* %s/usr/lib/python3.8/apres -r" % folder)
os.system("cp ../target/release/lib%s.so %s/usr/lib/ -p" % (name, folder))

os.system("tar --create --file \"%s.tar.gz\" %s" % (folder, folder))
os.system("rm \"%s\" -rf" % folder)

with open("PKGBUILD", "w") as fp:
    fp.write(build_pkgbuild(manifest))
os.system("makepkg -g -f -p PKGBUILD >> PKGBUILD")

os.system("rm src -rf")

os.chdir("../")
os.system("tar --create --file \"%s-dist.tar.gz\" %s/*" % (folder, name))
os.system("rm %s -rf" % name)
