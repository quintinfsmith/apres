import os, sys
import toml
import tarfile

def build_setup(manifest):
    author = manifest["package"]["authors"][0]
    author = author[0:author.find("<")].strip()
    author_email = manifest["package"]["authors"][0]
    author_email = author_email[author_email.find("<") + 1:author_email.find(">")]

    string_pairs = (
        ("name", "apres"),
        ("version", manifest["package"]["version"]),
        ("author", author),
        ("description", manifest["package"]["description"]),
        ("author_email", author_email),
        ("long_description_content_type", "text/markdown"),
        ("url", "https://burnsomni.net/git/apres"),
        ("python_requires", ">=3.7")
    )
    str_rep = ""
    for pair in string_pairs:
        str_rep += "\n%s=\"%s\"," % pair

    dependencies = ['cffi']

    output = """import setuptools
import platform
with open("README.md", "r") as fh:
    long_description = fh.read()
setuptools.setup(%s
    install_requires=%s,
    long_description=long_description,
    packages=setuptools.find_packages(),
    package_data={'apres': ["libapres_manylinux2014_x86_64.so", "libapres_manylinux2014_armv7l.so" ]},
    classifiers=[
        "Programming Language :: Python :: 3",
        "Programming Language :: Rust",
        "License :: OSI Approved :: GNU General Public License v2 or later (GPLv2+)",
        "Operating System :: POSIX :: Linux",
    ]
)
""" % (str_rep, str(dependencies))
    return output

file_path = os.path.realpath(__file__)
os.chdir(file_path)


manifest = toml.load("Cargo.toml")
version = manifest['package']['version']
description = manifest['package']['description']

folder = "pypibuild"
name = 'apres'

targets = [
    ("", "", "manylinux2014_x86_64"),
    ("--target armv7-unknown-linux-gnueabihf", "armv7-unknown-linux-gnueabihf", "manylinux2014_armv7l")
]

os.mkdir(folder)
os.mkdir("%s/%s" % (folder, name))
os.mkdir("%s/tests" % folder)

with open("%s/setup.py" % folder, "w") as fp:
    fp.write(build_setup(manifest))

os.system("cp src/python_bindings/* %s/%s -r" % (folder, name))
os.system("cp README.md %s/" % folder)
os.system("cp LICENSE %s/" % folder)

for target_opt, target_name, platname in targets:
    os.system('cargo build --release %s' % target_opt)
    os.system("cp target/%s/release/libapres_bindings.so %s/apres/libapres_%s.so" % (target_name, folder, platname))

os.chdir(folder)
os.system("python3 setup.py sdist bdist_wheel")

if "--zip" in sys.argv:
    os.chdir("../")
elif "--publish" in sys.argv:
    os.system("python3 -m twine upload dist/*.gz")
    os.chdir("../")
elif "--local" in sys.argv and target_name == "release":
    os.system("python3 setup.py install --prefix ~/.local/")
    os.chdir("../")
    #os.system("cp target/%s/release/libapres_bindings.so ~/.local/lib/libapres.so" % target_name)
else:
    input()
    os.chdir("../")
os.system("rm %s -rf" % folder)
os.system("cargo clean")
