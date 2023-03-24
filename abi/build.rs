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
            ],
            false,
            true,
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
        .compile(
            &[
                "readbot_proto/request.proto",
                "readbot_proto/response.proto",
            ],
            &["readbot_proto"],
        )
        .unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();
}
