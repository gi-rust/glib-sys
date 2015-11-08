This project provides a Rust FFI import crate for GLib.
Used as a dependency, it links the platform library as specified by
pkg-config module `glib-2.0`, and exports function declarations, data
types and constants needed to call functions of the C library API.

The source code for this library is generated from [GObject introspection][gi] data using [grust-gen][gen].

[gi]: https://wiki.gnome.org/Projects/GObjectIntrospection
[gen]: https://github.com/gi-rust/grust-gen
