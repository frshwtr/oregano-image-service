# oregano-image-service

Fetches an image from a given source and applies transformations.

## Features

### Resizing

Images can be resized to a given width and height. The mode of resizing can be controlled through options:

**fill (default)** - image will be resized to match the provided dimensions exactly
**Pad** - the image will be scaled to fit inside the provided dimensions, preserving the original aspect ratio. Any empty space after transforming will be filled with a background colour.

## Development Notes

It is useful to have a stored image available for testing purposes. A good example is [banana from wikipedia](https://upload.wikimedia.org/wikipedia/commons/d/de/Bananavarieties.jpg).