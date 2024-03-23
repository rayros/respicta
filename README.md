# Respicta - Image Resizer

<p align="center" width="100%">
    <img src="./images/logo_small.jpeg"> 
</p>

Image Resizer is a versatile project offering a library, command-line interface (CLI), and web service for resizing images and changing their formats seamlessly.

## Features

**Resize Images**: Effortlessly resize images to desired dimensions.

**Change Format**: Convert images to different formats such as JPEG, PNG, etc.

**CLI**: Intuitive command-line interface for quick resizing and format conversion.

**Web Service**: Host a web service to resize images on-the-fly.

## Convert

```bash
image-resizer convert --help
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