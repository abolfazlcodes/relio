use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::lifecycle::WindowGeometry;

const METADATA_FILE_NAME: &str = "lifecycle.json";
const MAX_METADATA_BYTES: u64 = 4 * 1024;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LifecycleMetadata {
    pub schema_version: u16,
    pub clean_exit: bool,
    pub window: Option<PersistedWindowGeometry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PersistedWindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
}

impl From<WindowGeometry> for PersistedWindowGeometry {
    fn from(value: WindowGeometry) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
            maximized: value.maximized,
        }
    }
}

impl From<PersistedWindowGeometry> for WindowGeometry {
    fn from(value: PersistedWindowGeometry) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
            maximized: value.maximized,
        }
    }
}

impl Default for LifecycleMetadata {
    fn default() -> Self {
        Self {
            schema_version: 1,
            clean_exit: true,
            window: None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    #[error("lifecycle metadata storage is unavailable")]
    Storage,
    #[error("lifecycle metadata is malformed or unsupported")]
    Malformed,
}

#[derive(Clone, Debug)]
pub struct LifecycleMetadataStore {
    directory: PathBuf,
}

impl LifecycleMetadataStore {
    pub fn open(directory: PathBuf) -> Result<Self, MetadataError> {
        std::fs::create_dir_all(&directory).map_err(|_| MetadataError::Storage)?;
        restrict_directory(&directory)?;
        Ok(Self { directory })
    }

    pub fn load(&self) -> Result<LifecycleMetadata, MetadataError> {
        let path = self.path();
        let Ok(file) = File::open(path) else {
            return Ok(LifecycleMetadata::default());
        };
        if file.metadata().map_err(|_| MetadataError::Storage)?.len() > MAX_METADATA_BYTES {
            return Err(MetadataError::Malformed);
        }
        let mut encoded = String::new();
        file.take(MAX_METADATA_BYTES + 1)
            .read_to_string(&mut encoded)
            .map_err(|_| MetadataError::Storage)?;
        let metadata: LifecycleMetadata =
            serde_json::from_str(&encoded).map_err(|_| MetadataError::Malformed)?;
        if metadata.schema_version != 1 {
            return Err(MetadataError::Malformed);
        }
        Ok(metadata)
    }

    pub fn mark_startup_unclean(&self) -> Result<LifecycleMetadata, MetadataError> {
        let previous = self.load()?;
        let mut active = previous.clone();
        active.clean_exit = false;
        self.persist(&active)?;
        Ok(previous)
    }

    pub fn mark_clean_exit(&self, window: Option<WindowGeometry>) -> Result<(), MetadataError> {
        self.persist(&LifecycleMetadata {
            schema_version: 1,
            clean_exit: true,
            window: window.map(Into::into),
        })
    }

    fn persist(&self, metadata: &LifecycleMetadata) -> Result<(), MetadataError> {
        let encoded = serde_json::to_vec(metadata).map_err(|_| MetadataError::Malformed)?;
        if encoded.len() as u64 > MAX_METADATA_BYTES {
            return Err(MetadataError::Malformed);
        }
        let temporary = self.directory.join("lifecycle.tmp");
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&temporary)
            .map_err(|_| MetadataError::Storage)?;
        restrict_file(&temporary)?;
        file.write_all(&encoded)
            .and_then(|()| file.sync_all())
            .map_err(|_| MetadataError::Storage)?;
        std::fs::rename(temporary, self.path()).map_err(|_| MetadataError::Storage)?;
        restrict_file(&self.path())
    }

    fn path(&self) -> PathBuf {
        self.directory.join(METADATA_FILE_NAME)
    }
}

#[cfg(unix)]
fn restrict_directory(path: &Path) -> Result<(), MetadataError> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))
        .map_err(|_| MetadataError::Storage)
}

#[cfg(unix)]
fn restrict_file(path: &Path) -> Result<(), MetadataError> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
        .map_err(|_| MetadataError::Storage)
}

#[cfg(windows)]
fn restrict_directory(_path: &Path) -> Result<(), MetadataError> {
    Ok(())
}

#[cfg(windows)]
fn restrict_file(_path: &Path) -> Result<(), MetadataError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn records_unclean_start_before_work_and_clean_exit_atomically() {
        let directory = std::env::temp_dir().join(format!("relio-lifecycle-{}", Uuid::now_v7()));
        let store = LifecycleMetadataStore::open(directory.clone()).expect("store");
        assert!(store.mark_startup_unclean().expect("startup").clean_exit);
        assert!(!store.load().expect("active marker").clean_exit);
        let geometry = WindowGeometry {
            x: 10,
            y: 20,
            width: 1120,
            height: 720,
            maximized: false,
        };
        store.mark_clean_exit(Some(geometry)).expect("clean exit");
        let saved = store.load().expect("saved");
        assert!(saved.clean_exit);
        assert_eq!(saved.window.map(WindowGeometry::from), Some(geometry));
        std::fs::remove_dir_all(directory).expect("remove fixture");
    }

    #[test]
    fn rejects_oversized_or_unknown_metadata() {
        let directory = std::env::temp_dir().join(format!("relio-lifecycle-{}", Uuid::now_v7()));
        let store = LifecycleMetadataStore::open(directory.clone()).expect("store");
        std::fs::write(
            directory.join(METADATA_FILE_NAME),
            vec![b'x'; MAX_METADATA_BYTES as usize + 1],
        )
        .expect("oversized fixture");
        assert!(matches!(store.load(), Err(MetadataError::Malformed)));
        std::fs::write(
            directory.join(METADATA_FILE_NAME),
            r#"{"schema_version":1,"clean_exit":true,"window":null,"secret":"no"}"#,
        )
        .expect("unknown fixture");
        assert!(matches!(store.load(), Err(MetadataError::Malformed)));
        std::fs::remove_dir_all(directory).expect("remove fixture");
    }
}
