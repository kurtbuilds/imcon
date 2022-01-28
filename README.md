<div id="top"></div>

<p align="center">
<a href="https://github.com/kurtbuilds/imcon/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/kurtbuilds/imcon.svg?style=flat-square" alt="GitHub Contributors" />
</a>
<a href="https://github.com/kurtbuilds/imcon/stargazers">
    <img src="https://img.shields.io/github/stars/kurtbuilds/imcon.svg?style=flat-square" alt="Stars" />
</a>
<a href="https://github.com/kurtbuilds/imcon/actions">
    <img src="https://img.shields.io/github/workflow/status/kurtbuilds/imcon/test?style=flat-square" alt="Build Status" />
</a>
<a href="https://crates.io/crates/imcon">
    <img src="https://img.shields.io/crates/d/imcon?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/imcon">
    <img src="https://img.shields.io/crates/v/imcon?style=flat-square" alt="Crates.io" />
</a>

</p>

# imcon

`imcon` is a library meant as a spiritual successor to ImageMagick, but with fewer dependencies and 
modern command line interface.

Right now it's in a very early stage of development, but it's a work in progress.

Supported file types:

- [x] PDF
- [ ] PNG
- [ ] JPEG
- [ ] TIFF
- [ ] GIF
- [ ] BMP
- [ ] ICO
- [ ] SVG

# Usage

    imcon --scale 2 ~/Downloads/multipage.pdf

This will scale the PDF to double the size, and create png files (the default for PDF) in your current directory.

```
$ ls
multipage0.png
multipage1.png
multipage2.png
multipage3.png
```

Read the help for more information.

# Installation

You need a copy of `pdfium` to be able to read PDF files. These
instructions makes that library available.

    git clone https://github.com/kurtbuilds/imcon
    cd imcon
    # Note this requires sudo, as it installs to /usr/local/, 
    # which (should be) owned by root. This script is tiny
    # enough that you can read it if you're especially worried 
    # about security.
    just install_with_library

> **Note:** if you don't already have `just`, install it with `cargo install just`.

Installation from cargo is not supported yet.

If you just need `imcon` and already have `pdfium`, you can install it with:

    git clone https://github.com/kurtbuilds/imcon
    cd imcon
    just install

# Roadmap

See the [open issues](https://github.com/othneildrew/Best-README-Template/issues) for a full list of proposed features (and known issues).

# Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request