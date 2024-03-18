use futures::io::Cursor;
use futures::{AsyncRead, AsyncReadExt};
use std::io;
use std::io::Read;

#[async_trait::async_trait]
pub trait FileSystem: Sync {
    async fn read_file(&self, path: &str) -> Result<Box<dyn AsyncRead + Send + Unpin>, io::Error>;
    async fn list_all_files_recursively(&self, path: &str) -> Result<Vec<String>, String>;
}

pub async fn convert_async_read_to_blocking_read(
    async_read: Box<dyn AsyncRead + Send + Unpin>,
) -> impl Read {
    let mut async_read = async_read;
    let mut buffer = Vec::new();
    let _ = async_read.read_to_end(&mut buffer).await;
    io::Cursor::new(buffer)
}

#[async_trait::async_trait]
impl FileSystem for quary_proto::FileSystem {
    async fn read_file(&self, path: &str) -> Result<Box<dyn AsyncRead + Send + Unpin>, io::Error> {
        if let Some(file) = self.files.get(path) {
            Ok(Box::new(Cursor::new(file.contents.clone())))
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File not found: {}", path),
            ))
        }
    }

    // TODO Should this return an iterator
    async fn list_all_files_recursively(&self, path: &str) -> Result<Vec<String>, String> {
        Ok(self
            .files
            .iter()
            .filter(|(file, _)| file.starts_with(path))
            .map(|(file, _)| file.to_string())
            .collect())
    }
}
