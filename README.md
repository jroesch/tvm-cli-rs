To build this example, first install Just.
```
cargo install just
```

To initialize the repository first run:
```
just init-tvm
```

To build:
```
just build
```

To test:
```
just test
```

To run:
```
just run
```
You can read the Justfile via `cat Jutfile` and see a full description of possible actions
via `just -l`.

CMake is configured to use Ninja as I find it is much faster then Make. You can tweak this
in the Justfile, and tweak the CMake config in `config/`.

There is an unfinished branch with direct installation of TVM via `build.rs` and CMake
but it is unfinished. This Justfile provides the necessary code to get started.
