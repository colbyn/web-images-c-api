# Web Images - C API

This project implements and exposes a C API of the rust image crate. 


# Build

```
cargo build --release
```

Notice the auto-generated C/C++ header files:
```shell
$ tree target
target/release
├── include
│   └── web_images_cabi.h
├── libweb_images_cabi.a
└── libweb_images_cabi.dylib
...
```

