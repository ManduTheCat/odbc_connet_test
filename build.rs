fn main() {
    println!("cargo:rustc-link-search=native=/root/unixODBC/lib");
    println!("cargo:rustc-link-lib=odbc");
}
