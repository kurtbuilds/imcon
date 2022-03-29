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
- [x] HEIC
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
    brew install libheif
    just install_with_library

> **Note:** if you don't already have `just`, install it with `cargo install just`.

Installation from cargo is not supported yet.

If you just need `imcon` and already have `pdfium`, you can install it with:

    git clone https://github.com/kurtbuilds/imcon
    cd imcon
    just install

# Roadmap

- [x] Add support for PDF
- [x] Add support for HEIC
- [x] Add support for PNG
- [x] Add support for JPEG
- [ ] Add support for TIFF
- [ ] Add support for TGA
- [ ] Add support for BMP
- [ ] Add support for ICO
- [ ] Add support for SVG
- [ ] Add support for WEBP
- [ ] Add support for command line flags
  - [ ] --in-place to replace input files in place.
  - [ ] --verbose to print out what's happening.
  - [ ] --lighten
  - [ ] --darken
  - [ ] --blur
- [ ] Add support for metadata (i.e. print metadata instead of creating the image)
- [ ] Support for using imcon as both a CLI and a library.
- [ ] Build a Python wrapper library.
- [ ] Build a Node wrapper library.
- [ ] Build a Ruby wrapper library.
- [ ] Write docs on target_width/height vs max_width/height.
- [ ] Create a Brew formula for installing Pdfium.
- [ ] Make build rules for libheif more flexible to diff versions of libheif.
- [ ] Benchmarks on various conversion & image manip operations, compared to other Rust (or other) libs.

# Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

# Development Notes

These are personal notes on solving problems in the repo, not written with public audience in mind.

- What are operations you want on scaling images?
    - Resize the image to a max side length.  (-W $size -H $size)
    - Resize the image to a specific width. (-w $size)
    - Resize the image to a specific height. (-h $size)

- You might also want to just print metadata.
  - width
  - height
  - # channels
  - exif data
  - # of pages
- Resize the image 


# Future examples

Create a transparent canvas. Prints to stdout. Defaults to .png.

    imcon -w 1024 -h 1024 \#000000ff > canvas.png  # can also be \#000f
    imcon -w 1024 -h 1024 \#000000ff -o canvas.jpg

    cat icon.png | imcon --stdin-format png --stdout-format jpg



    imcon canvas.png --dominant 5 



```python

# this is the brown ugly shit.
avg_patch = np.ones(shape=img.shape, dtype=np.uint8)*np.uint8(average)

indices = np.argsort(counts)[::-1]   
freqs = np.cumsum(np.hstack([[0], counts[indices]/float(counts.sum())]))
rows = np.int_(img.shape[0]*freqs)

dom_patch = np.zeros(shape=img.shape, dtype=np.uint8)
for i in range(len(rows) - 1):
    dom_patch[rows[i]:rows[i + 1], :, :] += np.uint8(palette[indices[i]])
    
fig, (ax0, ax1) = plt.subplots(1, 2, figsize=(12,6))
ax0.imshow(avg_patch)
ax0.set_title('Average color')
ax0.axis('off')
ax1.imshow(dom_patch)
ax1.set_title('Dominant colors')
ax1.axis('off')
plt.show(fig)


    DataSource {
        File(Path, Format),
        Memory(Bytes, Format),
        Image(::image::DynamicImage),
    }

    imcon::Image::open("foo.pdf").save_all_pages("foo.png")

    imcon::open_all("foo.pdf") -> Result<Vec<Image>
    
    either we load the data or it's deferred.
    
    imcon::open_all("foo.pdf") -> Result<Vec<Image>>
    
    imcon::ImageComputation::open("foo.pdf").resize(1024, 1024).save_to_path_template
    
    you can save from a pdf.... ok fine.
    
```
