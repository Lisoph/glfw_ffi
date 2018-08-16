# Pure FFI bindings to GLFW 3.2.1

This is a Rust crate that fully automatically builds the latest stable version of GLFW.
This crate does **not** provide high-level / safe / Rust-idiomatic bindings, only a raw FFI.
The GLFW source code is included as a git submodule.

## Prerequisites

- **CMake** for building GLFW.
- **Clang / libclang** is needed by bindgen in order to generate the bindings.

If both are installed, the crate should build without problems.
You can try running the `hello_world` example. If a black / empty window pops up, everything is good.

# Getting started

```rust
extern crate glfw_ffi;

use glfw_ffi::*;

fn main() {
    unsafe {
        if glfwInit() == 0 {
            return;
        }

        // You are ready to use GLFW.

        glfwTerminate();
    }
}
```

# License

This crate is licensed under the [MIT](./LICENSE) license. This does not apply to the GLFW project itself, please check GLFW's license for yourself.