extern crate bindgen;
extern crate cmake;

fn main() {
    let glfw_path = cmake::Config::new("glfw")
        .define("GLFW_BUILD_EXAMPLES", "OFF")
        .define("GLFW_BUILD_TESTS", "OFF")
        .define("GLFW_BUILD_DOCS", "OFF")
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        glfw_path.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=glfw3");
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
    }

    let bindings = bindgen::Builder::default()
        .header(format!(
            "{}",
            glfw_path
                .join("include")
                .join("GLFW")
                .join("glfw3.h")
                .display()
        ))
        .generate()
        .expect("Bindgen failed");
    bindings
        .write_to_file("bindings.rs")
        .expect("Failed to write to bidings.rs");
}
