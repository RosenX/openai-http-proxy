use std::process::Command;

use proto_builder_trait::tonic::BuilderAttributes;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .with_serde(&["request.RegisterInfo"], false, true, None)
        .compile(
            &[
                "../readbot_proto/request.proto",
                "../readbot_proto/response.proto",
            ],
            &["../readbot_proto"],
        )
        .unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();
}
