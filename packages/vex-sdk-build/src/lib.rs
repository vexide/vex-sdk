use std::{fs::File, io::Cursor, path::Path};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
struct Manifest {
    latest: String,
    catalog: Vec<String>,
}

pub fn link_sdk(version: &str) {
    let target = std::env::var("TARGET").expect("TARGET environment variable not set");
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV")
        .expect("CARGO_CFG_TARGET_ENV environment variable not set");
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR environment variable not set");

    if !target.contains("vex") {
        panic!("`vex-sdk-build` only supports building for VEX targets.")
    }

    let (cdn_base, folder_name, runtime_library_name) = match target_env.as_ref() {
        "v5" => ("https://content.vexrobotics.com/vexos/public/V5/vscode/sdk/cpp", "vexv5", "v5rt"),
        "exp" => ("https://content.vexrobotics.com/vexos/public/EXP/vscode/sdk/cpp", "vexexp", "exprt"),
        "iq2" => ("https://content.vexrobotics.com/vexos/public/IQ2/vscode/sdk/cpp", "vexiq2", "viq2rt"),
        _ => panic!("unsupported `target_vendor` value. `vex-sdk-build` only supports building for VEX targets.")
    };

    let manifest: Manifest = ureq::get(format!("{cdn_base}/manifest.json"))
        .call()
        .expect("failed to download VEX SDK manifest")
        .body_mut()
        .read_json()
        .expect("failed to parse VEX SDK manifest");

    if !manifest.catalog.contains(&version.to_string()) {
        panic!("{} is not a valid VEX SDK version", version);
    }

    let zip_data = ureq::get(format!("{cdn_base}/{version}.zip"))
        .call()
        .expect("failed to download VEX SDK")
        .body_mut()
        .read_to_vec()
        .expect("failed to read VEX SDK");

    let runtime_library_file = format!("lib{runtime_library_name}.a");
    let mut archive =
        zip::ZipArchive::new(Cursor::new(zip_data)).expect("failed to read VEX SDK zip archive");
    let runtime_library_path = format!("{version}/{folder_name}/{runtime_library_file}");

    std::io::copy(
        &mut archive.by_name(&runtime_library_path).expect(&format!(
            "VEX SDK is missing runtime library {runtime_library_path}"
        )),
        &mut File::create(&Path::new(&out_dir).join(runtime_library_file)).unwrap(),
    )
    .unwrap();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo:rustc-link-search=native={out_dir}");
    println!("cargo:rustc-link-lib=static={runtime_library_name}");
}
