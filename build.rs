fn main() {
    cc::Build::new()
        .file("src/time64.c")
        .compile("test");
}
