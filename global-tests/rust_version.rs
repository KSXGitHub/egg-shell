use cargo_metadata::MetadataCommand;
use cargo_toml::Manifest;
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use std::fs;

#[test]
fn sync() {
    let ws_root = MetadataCommand::new()
        .exec()
        .expect("Execute `cargo metadata`")
        .workspace_root;
    dbg!(&ws_root);
    let rust_toolchain = ws_root
        .join("rust-toolchain")
        .pipe(fs::read_to_string)
        .expect("Read the rust-toolchain file");
    let rust_toolchain = rust_toolchain.trim();
    dbg!(rust_toolchain);
    let members = ws_root
        .join("Cargo.toml")
        .pipe(Manifest::from_path)
        .expect("Read the top-level Cargo.toml file")
        .workspace
        .expect("Access Cargo.toml#workspace")
        .members;
    dbg!(&members);
    for member in &members {
        eprintln!();
        eprintln!("MEMBER: {member}");
        let manifest = ws_root
            .join(member)
            .join("Cargo.toml")
            .pipe(Manifest::from_path)
            .expect("Read a member Cargo.toml file");
        let rust_version = manifest.package().rust_version();
        dbg!(rust_version);
        assert_eq!(rust_version, Some(rust_toolchain));
    }
}
