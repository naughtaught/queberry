use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Plugin {
    pub name: String,
    pub js_path: PathBuf,
    pub manifest_path: PathBuf,
    pub plugin_dir: PathBuf,
}
