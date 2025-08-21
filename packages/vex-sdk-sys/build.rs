use std::{borrow::Cow, env::{self, home_dir}, fs::{self, DirEntry}, path::{Path, PathBuf}};

use serde::Deserialize;

fn main() {
    if cfg!(feature = "system-sdk") {
        search_system_sdk();
    }
}

fn search_system_sdk() {
    let vscode_data_dir = find_vscode_data_dir();
    let global_storage = vscode_data_dir.join("User").join("globalStorage");

    let extension_name = env::var("VEXCODE_EXTENSION_NAME")
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("vexrobotics.vexcode"));
    let vexcode_storage = global_storage.join(&*extension_name);

    let sdk_platform = target_product();
    let sdks = vexcode_storage.join("sdk").join("cpp").join(&*sdk_platform);

    let mut sdk_versions = fs::read_dir(&sdks).unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .into_iter()
        .filter(|e| e.file_type().unwrap().is_dir())
        .map(|e| e.file_name().into_string())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let sdk_version = match sdk_versions.len() {
        0 => panic!("Cannot find any installed VEX SDKs"),
        1 => sdk_versions.pop().unwrap(),
        _ => {
            let desired_sdk = env::var("VEX_SDK_VERSION")
                .unwrap_or_else(|_| pick_an_sdk(&sdk_versions));

            if sdk_versions.contains(&desired_sdk) {
                desired_sdk
            } else {
                pick_an_sdk(&sdk_versions)
            }
        }
    };

    let sdk = sdks.join(sdk_version);

    let platform = get_sdk_platform(&sdk);

    let sdk_platform_dir = sdk.join(platform);
    let newlib_dir = sdk_platform_dir.join("gcc").join("libs");

    println!("cargo::metadata=VEX_SDK_PATH={}", sdk_platform_dir.display());
    println!("cargo:rustc-link-lib=v5rt");
    println!("cargo:rustc-link-search={}", sdk_platform_dir.display());
    println!("cargo:rustc-link-search={}", newlib_dir.display());
}

fn find_vscode_data_dir() -> PathBuf {
    if let Ok(portable_path) = env::var("VSCODE_PORTABLE") {
        return PathBuf::from(portable_path).join("user-data");
    }

    let vscode_product_name = env::var("VSCODE_PRODUCT_NAME")
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("Code"));

    if let Ok(app_data) = env::var("VSCODE_APPDATA") {
        return PathBuf::from(app_data).join(&*vscode_product_name);
    }

    let platform_data_dir = match env::consts::OS {
        "windows" => {
            env::var("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    let user_profile = env::var("USERPROFILE")
                        .expect("$USERPROFILE must be defined on Windows");

                    PathBuf::from(user_profile)
                        .join("AppData")
                        .join("Roaming")
                })
        }
        "macos" => {
            let home = home_dir().unwrap();
            home.join("Library").join("Application Support")
        }
        "linux" => {
            env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    let home = home_dir().unwrap();
                    home.join(".config")
                })
        }
        platform => unimplemented!("cannot search for vex sdk on {platform}"),
    };

    platform_data_dir.join(&*vscode_product_name)
}

fn target_product() -> Cow<'static, str> {
    env::var("VEXCODE_PRODUCT_NAME")
        .map(Cow::Owned)
        .unwrap_or_else(|_| {
            let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
            Cow::Borrowed(match &*target_env {
                "v5" => "V5",
                env => unimplemented!("failed to detect VEXcode product for {env}")
            })
        })
}

fn pick_an_sdk(options: &[String]) -> ! {
    panic!(
        "Please set VEX_SDK_VERSION to one of the available SDK versions:\n{}",
        options.iter().map(|sdk| format!("- {sdk}")).collect::<Vec<_>>().join("\n"),
    )
}

fn get_sdk_platform(path: &Path) -> String {
    let manifest = path.join("manifest.json");
    let data = fs::read_to_string(manifest).expect("sdk manifest should be readable");
    let mut parsed: SdkManifest = serde_json::from_str(&data).expect("invalid sdk manifest");

    let Some(platform) = parsed.platforms.pop() else {
        panic!("SDK manifest is missing any platforms");
    };

    platform.name
}

#[derive(Deserialize)]
struct SdkManifest {
    platforms: Vec<SdkPlatform>,
}

#[derive(Deserialize)]
struct SdkPlatform {
    name: String,
}
