use std::process::Command;

use proto_builder_trait::tonic::BuilderAttributes;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .with_serde(
            &[
                "request.RegisterInfo",
                "request.RegisterRequest",
                "request.LoginRequest",
                "request.LoginInfo",
                "request.RefreshTokenRequest",
                "request.RefreshToken",
                "request.CreateFeedRequest",
                "request.FeedInfo",
            ],
            false,
            true,
            Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
        )
        .with_serde(
            &[
                "response.CreateFeedResponse",
                "response.UserFeed",
                "response.FeedProfile",
                "response.UserContent",
                "response.Content",
                "response.FetchContentResponse",
                "response.FecthFeedResponse",
            ],
            true,
            false,
            Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
        )
        .with_serde(
            &[
                "response.UserProfile",
                "request.ClientInfo",
                "response.Tokens",
            ],
            true,
            true,
            Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
        )
        .with_serde(&["response.AuthResponse"], true, false, None)
        .compile(&["proto/request.proto", "proto/response.proto"], &["proto"])
        .unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();
}
