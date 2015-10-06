# libgphoto2 Rust Bindings

The `gphoto2-sys` crate provides declarations and linkage for the `libgphoto2` C library. Following
the `*-sys` package conventions, the `gphoto2-sys` crate does not define higher-level abstractions
over the native `libgphoto2` library functions.

## Dependencies
In order to use the `gphoto2-sys` crate, you must have a Unix system with the `libgphoto2` library
installed where it can be found by `pkg-config`.


On Debian-based Linux distributions, install the `libgphoto2-dev` package:

```
sudo apt-get install libgphoto2-dev
```

On OS X, install `libgphoto2` with Homebrew:

```
brew install libgphoto2
```

## Usage
Add `gphoto2-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies]
gphoto2-sys = "0.1.0"
```

Import the `gphoto2_sys` crate and use the functions as they're defined in the native `libgphoto2`
library. See the [`libgphoto2` API documention](http://gphoto.org/doc/api/index.html) for more usage
information.

```rust
extern crate gphoto2_sys as gphoto2;
```

### OS X Usage
OS X opens cameras automatically when connected, which prevents other applications from opening the
camera device. When attempting to open a camera that is already opened by the operating system, you
will get an error message like the following:

```
Could not claim the USB device
```

To fix this, you have to kill the `PTPCamera` process after connecting a camera to your system:

```
killall PTPCamera
```

Each camera is opened with a separate instance of the `PTPCamera` application. If you have several
cameras connected, you may want to kill individual `PTPCamera` processes instead of using `killall`.

### Finding Help
Since `gphoto2-sys` does nothing more than export symbols from the native `libgphoto2` library, the
best source for help is the information already available for the native `libgphoto2`:

* [API Documentation](http://gphoto.org/doc/api/index.html)
* [Source Code](https://github.com/gphoto/libgphoto2)

## License
Copyright © 2015 David Cuddeback

Distributed under the [MIT License](LICENSE).

*Note:* By using this crate, your executable will link to the `libgphoto2` C library, which is
licensed under the [LGPL version 2.1](https://github.com/gphoto/libgphoto2/blob/master/COPYING).
