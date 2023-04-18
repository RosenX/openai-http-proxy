use std::process::Command;

use proto_builder_trait::tonic::BuilderAttributes;

fn main() {
    let protos = [
        "proto/model.proto",
        "proto/request.proto",
        "proto/response.proto",
    ];
    tonic_build::configure()
        .with_serde(
            &[
                "model.Feed",
                "model.FeedGroup",
                "model.FeedItem",
                "model.FeedUpdateRecord",
                "model.UserProfile",
                "model.SyncTimestamp",
                "model.JwtTokens",
                "model.ProLevel",
                "model.ClientInfo",
            ],
            true,
            true,
            Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
        )
        .with_serde(
            &[
                "response.Tokens",
                "response.ContentPullResponse",
                "response.ContentPushResponse",
                "response.AuthResponse",
            ],
            true,
            false,
            Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
        )
        .with_serde(
            &[
                "request.RegisterInfo",
                "request.RegisterRequest",
                "request.LoginRequest",
                "request.LoginInfo",
                "request.RefreshTokenRequest",
                "request.RefreshToken",
                "request.ContentPullRequest",
                "request.ContentPushRequest",
                "request.FeedItem",
                "request.FeedGroup",
                "request.Feed",
            ],
            false,
            true,
            Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
        )
        .out_dir("src/pb")
        .compile(&protos, &["proto/"])
        .unwrap();

    rerun(&protos);
    Command::new("cargo").args(["fmt"]).output().unwrap();
}

fn rerun(proto_files: &[&str]) {
    for proto_file in proto_files {
        println!("cargo:rerun-if-changed={}", proto_file);
    }
}
