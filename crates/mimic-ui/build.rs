fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        embed_resource::compile("src/assets/windows_manifest.rc", embed_resource::NONE);
    }
}
