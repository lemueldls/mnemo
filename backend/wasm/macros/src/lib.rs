use std::{collections::BTreeSet, ffi::OsStr, fs, path::PathBuf, str::FromStr};

use proc_macro::TokenStream;
use quote::quote;
use semver::Version;
use serde::Deserialize;
use walkdir::WalkDir;

/// The `package` key in the manifest.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
struct PackageInfo {
    name: String,
    version: Version,
    entrypoint: String,
    authors: Vec<String>,
    license: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    keywords: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compiler: Option<Version>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    exclude: Vec<String>,
}

#[proc_macro]
pub fn packages(_: TokenStream) -> TokenStream {
    let mut packages = Vec::new();

    let pkgs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("packages/packages")
        .canonicalize()
        .unwrap();

    for entry in WalkDir::new(pkgs_dir).min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        let ns_path = entry.path();

        let namespace = ns_path.file_name().unwrap().to_str().unwrap();

        for entry in WalkDir::new(ns_path).min_depth(1).max_depth(1) {
            let entry = entry.unwrap();
            let pkg_name = entry.path();

            let name = pkg_name.file_name().unwrap().to_str().unwrap();

            for entry in WalkDir::new(pkg_name).min_depth(1).max_depth(1) {
                let entry = entry.unwrap();
                let pkg_version = entry.path();

                let version = pkg_version.file_name().unwrap().to_str().unwrap();

                let (paths, contents) = WalkDir::new(&pkg_version)
                    .into_iter()
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        let file_path = entry.path();

                        file_path.extension().and_then(|ext| {
                            if ext == "typ" || ext == "toml" {
                                let path = file_path
                                    .strip_prefix(&pkg_version)
                                    .unwrap()
                                    .to_string_lossy()
                                    .to_string();
                                let content = fs::read_to_string(file_path).unwrap();

                                Some((path, content))
                            } else {
                                None
                            }
                        })
                    })
                    .unzip::<_, _, Vec<_>, Vec<_>>();

                let package = format!("@{namespace}/{name}:{version}");

                // if name == "cetz" || name == "oxifmt" {
                packages.push(
                    quote!((#package, ::std::boxed::Box::from_iter([#((#paths, #contents)),*]))),
                );
                // }
            }

            // let versions = WalkDir::new(pkg_name)
            //     .min_depth(1)
            //     .max_depth(1)
            //     .into_iter()
            //     .filter_map(|entry| {
            //         let entry = entry.ok()?;
            //         let version = entry.path().file_name()?.to_str()?;
            //         let version = Version::parse(version).ok()?;

            //         Some(version)
            //     })
            //     .collect::<BTreeSet<_>>();
            // let version = versions.last().unwrap();

            // let root = pkg_name.join(version.to_string());

            // let (paths, contents) = WalkDir::new(&root)
            //     .into_iter()
            //     .filter_map(|entry| {
            //         let entry = entry.ok()?;
            //         let file_path = entry.path();

            //         file_path.extension().and_then(|ext| {
            //             if ext == "typ" || ext == "toml" {
            //                 let path = file_path
            //                     .strip_prefix(&root)
            //                     .unwrap()
            //                     .to_string_lossy()
            //                     .to_string();
            //                 let content = fs::read_to_string(file_path).unwrap();

            //                 Some((path, content))
            //             } else {
            //                 None
            //             }
            //         })
            //     })
            //     .unzip::<_, _, Vec<_>, Vec<_>>();

            // let package = format!("@{namespace}/{name}:{version}");

            // if name == "cetz" || name == "oxifmt" {
            //     dbg!(name);

            //     packages.push(
            //         quote!((#package, ::std::boxed::Box::from_iter([#((#paths, #contents)),*]))),
            //     );
            // }
        }
    }

    let expanded = quote!([#(#packages),*]);

    TokenStream::from(expanded)
}
