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
            None,
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
            None,
        )
        .with_serde(
            &[
                "response.UserProfile",
                "request.ClientInfo",
                "response.Tokens",
            ],
            true,
            true,
            None,
        )
        .with_serde(&["response.AuthResponse"], true, false, None)
        .with_type_attributes(
            &["request.FetchContentRequest"],
            &[r#"#[derive(FromForm)]"#],
        )
        .compile(&["proto/request.proto", "proto/response.proto"], &["proto"])
        .unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();
}
