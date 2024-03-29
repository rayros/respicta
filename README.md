# Respicta - Image Resizer

<p align="center" width="100%">
    <img src="./images/logo_small.jpeg"> 
</p>

Respicta is a versatile project offering a library, command-line interface (CLI), and web service for resizing images and changing their formats seamlessly.

## Features

**Resize Images**: Effortlessly resize images to desired dimensions.

**Change Format**: Convert images to different formats such as JPEG, PNG, etc.

**CLI**: Intuitive command-line interface for quick resizing and format conversion.

**Web Service**: Host a web service to resize images on-the-fly.

## Convert

```bash
docker run --rm -v ./:/images rayros/respicta convert --help
```

```plaintext
Convert images from one format to another

Usage: image-resizer convert [OPTIONS] <INPUT_PATH> <OUTPUT_PATH>

Arguments:
  <INPUT_PATH>   Input image path
  <OUTPUT_PATH>  Output image path

Options:
      --help             
  -w, --width <WIDTH>    Width of the output image If not set, the width will be the same as the input image
  -h, --height <HEIGHT>  Height of the output image If not set, the height will be the same as the input image


Examples:

  image-resizer convert --width 100 --height 100 input.jpg output.jpg
```

## Server

```bash
docker run --rm rayros/respicta server --help
```

```plaintext
Start a server to convert images

Usage: respicta server [OPTIONS]

Options:
  -a, --address <ADDRESS>  Address to bind the server to (default: 0.0.0.0:3000)
  -l, --limit <LIMIT>      Maximum file size in bytes (default: 10MB)
  -h, --help               Print help
```

---

##### Program utilized in:

- [mamrzeczy.pl - Free Classifieds in Poland](https://mamrzeczy.pl)