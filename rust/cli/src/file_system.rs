use futures::io::AsyncRead;
use quary_core::file_system::FileSystem;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{fs, io};

struct AsyncReadAdapter<R> {
    inner: R,
}

impl<R: Read + Send + 'static> AsyncRead for AsyncReadAdapter<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match self.inner.read(buf) {
            Ok(n) => Poll::Ready(Ok(n)),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Poll::Pending,
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

impl<R: Read + Send + 'static> Unpin for AsyncReadAdapter<R> {}

fn convert_read_to_async_read<R: Read + Send + 'static>(
    read: R,
) -> Box<dyn AsyncRead + Send + Unpin> {
    Box::new(AsyncReadAdapter { inner: read })
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

#[async_trait::async_trait]
impl FileSystem for LocalFS {
    async fn read_file(&self, path: &str) -> Result<Box<dyn AsyncRead + Send + Unpin>, io::Error> {
        let mut full_path = self.root.clone();
        full_path.push(path);
        let file = File::open(full_path)?;
        Ok(Box::new(convert_read_to_async_read(file)))
    }

    async fn list_all_files_recursively(&self, path: &str) -> Result<Vec<String>, String> {
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
