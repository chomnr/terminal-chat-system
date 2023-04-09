use std::{env, path::PathBuf};

fn main()  {
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .type_attribute(".", "#[derive(serde::Deserialize)]")
        .compile(&["proto/chat/chat.proto"], &["proto"])
        .unwrap();
}