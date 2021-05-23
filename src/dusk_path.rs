use std::path::PathBuf;

pub struct GamePath {
    path: PathBuf,
}

/// GamePath is a convenient path operation in dusk engine.
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
    /// returns a source's directory in the path.
    pub fn source(self, name: &str) -> PathBuf {
        self.path.join(name)
    }
}

impl Clone for GamePath {
    fn clone(&self) -> Self {
        GamePath {path: self.path.clone()}
    }
}
