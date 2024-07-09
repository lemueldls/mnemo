pub mod bundler;

use std::collections::HashMap;

use bundler::model::IndexPackageInfo;
use serde::{Deserialize, Serialize};

// name	"a2c-nums"
// version	"0.0.1"
// entrypoint	"src/lib.typ"
// authors	[…]
// license	"MIT"
// description	"Convert a number to Chinese"
// repository	"https://github.com/soarowl/a2c-nums.git"
// keywords	[…]
// compiler	"0.10.0"
// exclude	[…]
// updatedAt	1704708827

#[tokio::main]
pub async fn fetch_packages() -> Result<Vec<IndexPackageInfo>, reqwest::Error> {
    reqwest::get("https://packages.typst.org/preview/index.json")
        .await?
        .json::<Vec<IndexPackageInfo>>()
        .await
}

#[test]
fn test_fetch_packages() {
    fetch_packages().unwrap();
}
