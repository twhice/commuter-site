fn main() {
    std::process::Command::new("leptosfmt")
        .arg("./**/*.rs")
        .spawn()
        .unwrap();
}
