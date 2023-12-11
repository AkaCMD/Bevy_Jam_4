extern crate embed_resource;

#[cfg(target_os = "windows")]
fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("icon.rc", embed_resource::NONE);
    }
}