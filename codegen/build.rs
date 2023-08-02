use std::{io::Write, process::Command};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    components(schemas(
        abi::JwtTokens,
        abi::ProLevel,
        abi::ClientInfo,
        abi::UserProfile,
        abi::FeedGroup,
        abi::Feed,
        abi::FeedItem,
        abi::FeedUpdateRecord,
        abi::SyncTimestamp,
        abi::FeedTypeServer,
        abi::RegisterInfo,
        abi::LoginInfo,
        abi::LoginRequest,
        abi::RegisterRequest,
        abi::RefreshTokenRequest,
        abi::ContentPullRequest,
        abi::ContentPushRequest,
        abi::AuthResponse,
        abi::ContentPullResponse,
        abi::ContentPushResponse,
        abi::SubscribeFeedRequest,
        abi::SubscribeFeedResponse,
        abi::FeedInfo,
        abi::ModifyPasswordRequest
    ),),
    tags()
)]
struct ApiDoc;

fn main() {
    let json_file_name = "openapi.json";
    std::fs::remove_file(json_file_name).unwrap_or_default();

    let doc = ApiDoc::openapi();
    let open_api = doc.to_pretty_json();

    match open_api {
        // ok then save to file
        Ok(open_api) => {
            let mut file = std::fs::File::create(json_file_name).unwrap();
            file.write_all(open_api.as_bytes()).unwrap();
        }
        // error then print to console
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let output_path = "code";
    let model_path = format!("{}/lib/model", output_path);
    let template_path = "dart";
    let josn_helper_path = "dart/json.dart";
    std::fs::remove_dir_all(output_path).unwrap_or_default();

    // use openapi generator to generate client code
    let mut cmd = Command::new("openapi-generator");
    cmd.arg("generate")
        .arg("-i")
        .arg(json_file_name)
        .arg("-g")
        .arg("dart")
        .arg("-o")
        .arg(output_path)
        .arg("-t")
        .arg(template_path)
        .arg("--global-property=models,modelDocs=false,modelTests=false")
        .output()
        .unwrap();

    // create a 'index.dart' file in model folder
    std::fs::File::create(format!("{}/index.dart", model_path)).unwrap();
    // copy json helper to model folder
    std::fs::copy(josn_helper_path, format!("{}/json.dart", model_path)).unwrap();

    // for all file in output_path except 'index.dart', write export to 'index.dart'
    for entry in std::fs::read_dir(model_path.clone()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name != "index.dart" {
                let mut file = std::fs::OpenOptions::new()
                    .append(true)
                    .open(format!("{}/index.dart", model_path))
                    .unwrap();
                file.write_all(format!("export '{}'; \n", file_name).as_bytes())
                    .unwrap();
            }
        }
    }

    let target_path = "../../readbot_app/lib/service/base/abi";
    // remove target path
    std::fs::remove_dir_all(target_path).unwrap_or_default();
    // move model folder to target path
    std::fs::rename(model_path, target_path).unwrap();
    // remove output path
    std::fs::remove_dir_all(output_path).unwrap_or_default();
}
