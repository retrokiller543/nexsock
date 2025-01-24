fn main() {
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=public");
}
