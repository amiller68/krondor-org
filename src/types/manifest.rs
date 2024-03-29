use std::collections::BTreeMap;
use std::path::PathBuf;

use cid::Cid;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

use super::object::Object;

/// Manifest: describes the state of content
/// - objects: a set of Objects that comprise website content
/// - previous_root: a cid pointing back to the previous version of the manifest
/// - version: version information on the crate
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Manifest {
    objects: BTreeMap<PathBuf, Object>,
    previous_root: Cid,
    version: Version,
}

impl Manifest {
    pub fn objects(&self) -> &BTreeMap<PathBuf, Object> {
        &self.objects
    }

    #[allow(dead_code)]
    pub fn object_by_cid(&self, cid: &Cid) -> Option<(&PathBuf, &Object)> {
        self.objects.iter().find(|(_, object)| object.cid() == cid)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Manifest {
    pub fn set_previous_root(&mut self, cid: Cid) {
        self.previous_root = cid;
    }

    pub fn insert_object(&mut self, path: &Path, object: &Object) {
        self.objects.insert(path.to_path_buf(), object.clone());
    }

    pub fn remove_object(&mut self, path: &PathBuf) {
        self.objects.remove(path);
    }

    pub fn get_object_mut(&mut self, path: &PathBuf) -> Option<&mut Object> {
        self.objects.get_mut(path)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub build_profile: String,
    pub build_features: String,
    pub repo_version: String,
    pub version: String,
}

impl Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl Version {
    pub fn new() -> Self {
        Self {
            build_profile: env!("BUILD_PROFILE").to_string(),
            build_features: env!("BUILD_FEATURES").to_string(),
            repo_version: env!("REPO_VERSION").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}
