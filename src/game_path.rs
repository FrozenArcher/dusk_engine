use std::path::PathBuf;

/// GamePath is a convenient path operation in dusk engine.
/// use this so that you don't need to use the crate fins_folder.
///
/// resolved problems caused by different path formats in different OSs.
///
/// # Panics 
/// if the argument of the "folder" method refers to a missing directory,
/// the program will panic.
/// ***
/// **usages**:
/// ```
/// use dusk_engine::game_path::GamePath;
/// // for the fn open, you don't need to use PathBuf yourself.
/// use std::path::PathBuf;  
/// // a sample function,you don't need to write this yourself.
/// fn open(path: PathBuf) {}
///
/// let some_path = GamePath::folder("test")
///     .sub("test_assets")
///     .sub("sub_dir")
///     .file("sample.txt");
///
/// open(some_path);
/// ```
pub struct GamePath {
    path: PathBuf,
}

impl GamePath {
    /// refers to a directory.
    /// 
    /// # Panics
    ///
    /// if argument `name` refers to a non-exist directory, 
    /// the program will panic.
    pub fn folder(name: &str) -> Self {
        let path = find_folder::Search::KidsThenParents(3, 5)
            .for_folder(name)
            .unwrap();
        GamePath{path}
    }
    /// joins a subpath to the GamePath.
    pub fn sub(mut self, name: &str) -> Self {
        self.path = self.path.join(name);
        self
    }
    /// returns a file's directory in the path.
    pub fn file(self, name: &str) -> PathBuf {
        self.path.join(name)
    }
}

impl Clone for GamePath {
    fn clone(&self) -> Self {
        GamePath {path: self.path.clone()}
    }
}
