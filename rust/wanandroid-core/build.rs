fn main() {
    println!("cargo:rerun-if-changed=src/wanandroid.udl");
    uniffi::generate_scaffolding("src/wanandroid.udl")
        .expect("UniFFI scaffolding generation failed");
}
