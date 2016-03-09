# photoshell_thumbnailer

`photoshell_thumbnailer` is a thumbnailer for RAW images.

## Building

```
cargo build --release
```

After building, the `photoshell_thumbnailer` binary will be in `target/release/`

## Usage

```
photoshell_thumbnailer input_file output_file
```

## Installation for Nautilus

Make sure the `photoshell_thumbnailer` binary exists on your path and copy `photoshell.thumbnailer` to `/usr/share/thumbnailers/`. You may also need to change the Nautilus settings to allow thumbnails for larger files. Go to `Preferences -> Preview -> Only for files smaller than:` and set it to a value that's large enough.
