use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub trait FileSystem {
    fn read_file(&self, path: &str) -> Result<Box<dyn Read>, io::Error>;

    fn list_all_files_recursively(&self, path: &str) -> Result<Vec<String>, String>;
}

impl FileSystem for quary_proto::FileSystem {
    fn read_file(&self, path: &str) -> Result<Box<dyn Read>, io::Error> {
        self.files
            .get(path)
            .map(|file| {
                Box::new(io::Cursor::new(Vec::from(file.contents.clone()))) as Box<dyn Read>
            })
            .ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File not found: {}", path),
            ))
    }

    // TODO Should this return an iterator
    fn list_all_files_recursively(&self, path: &str) -> Result<Vec<String>, String> {
        Ok(self
            .files
            .iter()
            .filter(|(file, _)| file.starts_with(path))
            .map(|(file, _)| file.to_string())
            .collect())
    }
}

#[derive(Debug, Clone)]
pub struct LocalFS {
    root: PathBuf,
}

impl LocalFS {
    pub fn new(root_path: PathBuf) -> Self {
        LocalFS { root: root_path }
    }
}

impl FileSystem for LocalFS {
    fn read_file(&self, path: &str) -> Result<Box<dyn Read>, io::Error> {
        let mut full_path = self.root.clone();
        full_path.push(path);
        let file = File::open(full_path)?;
        Ok(Box::new(file))
    }

    fn list_all_files_recursively(&self, path: &str) -> Result<Vec<String>, String> {
        let mut full_path = self.root.clone();
        full_path.push(path);
        let mapped_paths = self
            .recursive_file_search(&full_path)
            .map_err(|e| e.to_string())?;
        mapped_paths
            .iter()
            .map(|p| {
                Ok(p.strip_prefix(&self.root)
                    .map_err(|e| e.to_string())?
                    .to_str()
                    .ok_or("Invalid UTF-8 in path".to_string())?
                    .to_string())
            })
            .collect()
    }
}

impl LocalFS {
    fn recursive_file_search(&self, dir_path: &Path) -> io::Result<Vec<PathBuf>> {
        let mut files = vec![];

        if dir_path.is_dir() {
            for entry in fs::read_dir(dir_path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    files.extend(self.recursive_file_search(&path)?);
                } else if path.is_file() {
                    files.push(path);
                }
            }
        }

        Ok(files)
    }
}
