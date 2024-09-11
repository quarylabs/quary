/// Helper functions for the proto work that needs help
use crate::rpc_proto_scaffolding::Writer;
use quary_proto::FileSystem;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) fn setup_file_mocks() -> (Writer, Rc<RefCell<HashMap<String, String>>>) {
    let written_files = Rc::new(RefCell::new(HashMap::new()));
    let writer: Writer = Box::new({
        let written_files = Rc::clone(&written_files);
        move |path, content| {
            written_files.borrow_mut().insert(path, content);
            Box::pin(async move { Ok(()) })
        }
    });
    (writer, written_files)
}

pub(crate) fn files_to_file_system(files: Vec<(&str, &str)>) -> FileSystem {
    FileSystem {
        files: files
            .into_iter()
            .map(|(name, contents)| {
                let contents = contents.to_string();
                (
                    name.to_string(),
                    quary_proto::File {
                        name: name.to_string(),
                        contents: prost::bytes::Bytes::from(contents),
                    },
                )
            })
            .collect(),
    }
}
