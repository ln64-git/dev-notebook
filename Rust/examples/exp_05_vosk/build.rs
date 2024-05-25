use std::env;

fn main() {
    // Specify the path to your libraries. Adjust this path as necessary.
    let lib_path = "/usr/lib/";

    // Tell cargo to tell rustc to link the system shared libraries located at `lib_path`
    println!("cargo:rustc-link-search={}", lib_path);

    // If you know the names of the libraries you want to link, you can specify them here
    // For example, to link against a library named `libexample.a`, uncomment the next line:
    // println!("cargo:rustc-link-lib=static=example");

    // Optionally, you can set environment variables if needed
    env::set_var("RUSTFLAGS", format!("-L {}", lib_path));
}
