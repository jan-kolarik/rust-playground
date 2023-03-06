trait FileMetadata {
    fn file_exists(&self) -> bool;
    fn is_writeable_file(&self) -> bool;
    fn is_readable_file(&self) -> bool;
}

impl FileMetadata for std::path::Path {
    fn file_exists(&self) -> bool {
        self.exists() && self.is_file()
    }

    fn is_writeable_file(&self) -> bool {
        let read_only = || std::fs::metadata(self).unwrap().permissions().readonly();
        self.file_exists() && !read_only()
    }

    // Assuming from docs that Path::exists() returns false on access permission problems
    fn is_readable_file(&self) -> bool {
        self.file_exists()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::Path;

    use super::*;

    struct TestFile {
        path: Box<Path>
    }

    impl Drop for TestFile {
        fn drop(&mut self) {
            if self.path.exists() {
                std::fs::remove_file(&self.path);
            }
        }
    }

    impl TestFile {
        fn new(path: Box<Path>) -> TestFile {
            TestFile { path }
        }

        fn create_file(&self, read_only: bool) {
            let file = File::create(&self.path).unwrap();
            if read_only {
                let mut permissions = file.metadata().unwrap().permissions();
                permissions.set_readonly(true);
                file.set_permissions(permissions);
            }
        }
    }

    #[test]
    fn output_dirs() {
        let paths = std::fs::read_dir("./").unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }
    }

    #[test]
    fn not_existing_file() {
        let unknown_path = Path::new("unknown");
        assert!(!unknown_path.file_exists());
        assert!(!unknown_path.is_readable_file());
        assert!(!unknown_path.is_writeable_file());
    }

    #[test]
    fn not_file() {
        let dir_path = Path::new("src");
        assert!(!dir_path.file_exists());
        assert!(!dir_path.is_readable_file());
        assert!(!dir_path.is_writeable_file());
    }

    #[test]
    fn not_writeable_file() {
        let new_file_path = Path::new("readonly");
        let new_file = TestFile::new(new_file_path.into());
        new_file.create_file(true);
        assert!(!new_file_path.is_writeable_file());
        assert!(new_file_path.is_readable_file());
    }

    #[test]
    fn writeable_file() {
        let new_file_path = Path::new("writeable");
        let new_file = TestFile::new(new_file_path.into());
        new_file.create_file(false);
        assert!(new_file_path.is_writeable_file());
        assert!(new_file_path.is_readable_file());
    }
}