use std::{env, path::PathBuf};

fn main()  {
    tonic_build::configure()
        .compile(&["proto/chat/chat.proto"], &["proto"])
        .unwrap();
}