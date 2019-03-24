extern crate bindgen;
extern crate cmake;

fn main() {
    let glfw_path = cmake::Config::new("glfw")
        .define("GLFW_BUILD_EXAMPLES", "OFF")
        .define("GLFW_BUILD_TESTS", "OFF")
        .define("GLFW_BUILD_DOCS", "OFF")
        .define("GLFW_INCLUDE_NONE", "1")
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        glfw_path.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=glfw3");
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=shell32");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
    }

    let builder = bindgen::Builder::default()
        .clang_arg("-D GLFW_INCLUDE_NONE")
        .header(format!(
            "{}",
            glfw_path
                .join("include")
                .join("GLFW")
                .join("glfw3.h")
                .display()
        ));
        println!("Flags:");
        builder.command_line_flags().into_iter().for_each(|f| {
            println!("{}", f);
        });
        let bindings = builder
        .generate()
        .expect("Bindgen failed");
    bindings
        .write_to_file("bindings.rs")
        .expect("Failed to write to bidings.rs");
}
