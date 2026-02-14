fn main() {
    println!("cargo:rustc-link-search=native=examples/libs");
    println!("cargo:rustc-link-lib=static=string");
    println!("cargo:rustc-link-lib=static=math");
    println!("cargo:rustc-link-lib=static=net");
}