# mosapic

_A command line tool to turn a picture into a mosaic made of other pictures_

---

# Install

Just download the `mosapic` binary from the Releases section to your `/usr/local/bin` and make it executable.

# Usage

## Preparing the tiles

This command will parse your pictures directory, crop the pictures into squares and store them as 100x100 pixels pictures in 
the directory of your choice.

```bash
mosapic crop <SOURCE_DIRECTORY> [<DESTINATION_DIRECTORY>]
```

| Argument              | Description                                                           | Required | Default      |
| --------------------- | --------------------------------------------------------------------- | -------- | ------------ |
| SOURCE_DIRECTORY      | The directory where the pictures you want to use as tiles are located | Yes      |              |
| DESTINATION_DIRECTORY | The directory where to store the tiles                                | No       | /tmp/mosapic |

## Building the picture

```bash
mosapic make <PICTURE> [<TILES_PER_SIDE> [<TILES_DIRECTORY>]]
```

| Argument        | Description                              | Required | Default      |
|-----------------|------------------------------------------|----------|--------------|
| PICTURE         | The picture to turn into a mosaic        | Yes      |              |
| TILES_PER_SIDE  | The amount of tiles per side of the pic  | No       | 10           |
| TILES_DIRECTORY | The directory where the tiles are stored | No       | /tmp/mosapic |

### Note

The picture will be built by pieces of 10x10 tiles, which means some pictures can be reused several times, but only once per piece.
