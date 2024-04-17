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

## Supported conversions 

- Gif to WebP
- Jpeg to WebP
- Png to Jpeg
- Png to WebP

## CLI

### Convert

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

### Server

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

# As a library

```rust
use respicta::convert;

fn main() {
    convert(&respicta::Config {
        input_path: &"images/logo.jpeg".into(),
        output_path: &"images/logo_small.jpeg".into(),
        width: Some(200),
        height: Some(200),
    })
    .unwrap();
}
```

# Kubernetes example use (server)

How to use respicta inside pod for your custom resizer service.

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  labels:
    app: my-resizer-service
  name: my-resizer-service
spec:
  replicas: 1
  selector:
    matchLabels:
      app: my-resizer-service
  template:
    metadata:
      labels:
        app: my-resizer-service
    spec:
      containers:
        - image: rayros/respicta
          name: respicta
          args: ["server", "--address", "0.0.0.0:4000"]
        - image: main-app-image:latest
          name: main-app
          ports:
            - containerPort: 2137
          env:
            - name: RESPICTA_HREF
              value: http://localhost:4000
```

# WIP

- command-server - send cli commands via http

---

##### Program utilized in:

- [mamrzeczy.pl - Free Classifieds in Poland](https://mamrzeczy.pl)
