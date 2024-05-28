# [Box blur](https://en.wikipedia.org/wiki/Box_blur) implementation.

This is not the fastest implementation and it's not intended to be. It's intended to be a simple implementation that is easy to understand made for me to study image processing.

This implementation can replicate the results of the gaussian blur implementation of the image library that I'm using. The difference is that the gaussian blur implementation is faster and the box blur implementation is simpler and slower.

## Usage

```
cargo run -- <path> <iterations> [ratio]
```

Where `<path>` is the path to the image file and `<iterations>` is the number of iterations that the box blur algorithm will run on the image. The optional `[ratio]` parameter is the ratio of the box blur algorithm. The default value is 1.

## Image example

Original image:

![Original image](./example.jpg)

After 30 iterations:

![Blurred image](./example_blurred.jpg)
