use crate::file_system::FileSystem;
use async_trait::async_trait;
use futures::AsyncRead;
use std::collections::{BTreeSet, HashMap};
use std::io;

/// Adds overrides to a file system. If a file is requested that has an override, the override is
/// returned instead of the actual file.
pub struct OverrideFileSystem {
    fs: Box<dyn FileSystem>,
    overrides: HashMap<String, String>,
}

impl OverrideFileSystem {
    pub fn new(fs: Box<dyn FileSystem>) -> Self {
        Self {
            fs,
            overrides: HashMap::new(),
        }
    }

    pub fn add_override(&mut self, path: &str, content: &str) {
        self.overrides.insert(path.to_string(), content.to_string());
    }
}

#[async_trait]
impl FileSystem for OverrideFileSystem {
    async fn read_file(&self, path: &str) -> Result<Box<dyn AsyncRead + Send + Unpin>, io::Error> {
        if let Some(content) = self.overrides.get(path) {
            let content = content.clone();
            Ok(Box::new(futures::io::Cursor::new(content.into_bytes())))
        } else {
            self.fs.read_file(path).await
        }
    }

    async fn list_all_files_recursively(&self, path: &str) -> Result<Vec<String>, String> {
        let mut overrides = self
            .overrides
            .iter()
            .filter(|(k, _)| k.starts_with(path))
            .map(|(k, _)| k.clone())
            .collect::<BTreeSet<String>>();
        let files = self.fs.list_all_files_recursively(path).await?;
        // make sure unique
        overrides.extend(files.into_iter());
        Ok(overrides.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_system::MockFileSystem;
    use futures::AsyncReadExt;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_override_file_system() {
        let mut fs = MockFileSystem::new();

        fs.expect_read_file()
            .times(1)
            .with(eq("file2"))
            .returning(|_| Ok(Box::new(futures::io::Cursor::new("file2".as_bytes()))));
        fs.expect_list_all_files_recursively()
            .times(1)
            .with(eq(""))
            .returning(|_| Ok(vec!["file1".to_string(), "file2".to_string()]));
        fs.expect_list_all_files_recursively()
            .times(1)
            .with(eq("dir1"))
            .returning(|_| Ok(vec![]));

        let mut ofs = OverrideFileSystem::new(Box::new(fs));
        ofs.add_override("file1", "override1");
        ofs.add_override("dir1/file3", "override2");

        let mut file_1 = String::new();
        ofs.read_file("file1")
            .await
            .unwrap()
            .read_to_string(&mut file_1)
            .await
            .unwrap();
        assert_eq!("override1", &file_1);

        let mut file_2 = String::new();
        ofs.read_file("file2")
            .await
            .unwrap()
            .read_to_string(&mut file_2)
            .await
            .unwrap();
        assert_eq!("file2", &file_2);

        let mut file_3 = String::new();
        ofs.read_file("dir1/file3")
            .await
            .unwrap()
            .read_to_string(&mut file_3)
            .await
            .unwrap();
        assert_eq!(&file_3, "override2");

        assert_eq!(
            ofs.list_all_files_recursively("").await.unwrap(),
            vec!["dir1/file3", "file1", "file2"]
        );
        assert_eq!(
            ofs.list_all_files_recursively("dir1").await.unwrap(),
            vec!["dir1/file3"]
        );
    }
}
