use std::path::Path;

pub trait Source<'a> {
    fn get_label(&self) -> &String;
    fn set_label(&mut self, label: &String);
    fn get_config_path(&self) -> &'a Path;
    fn set_config_path(&mut self, path: &'a Path);
    fn download(&self);
    fn run(&self);
}

/// The Registry containing the sources' main functions that can be called.
pub struct SourceRegistry<'a> {
    sources: Vec<Box<dyn Source<'a>>>,
}

impl<'a> SourceRegistry<'a> {
    pub fn new() -> Self {
        Self { sources: vec![] }
    }

    pub fn add_source<S>(&mut self, source: S)
    where
        S: Source<'a> + 'static
    {
        self.sources.push(Box::new(source));
    }

    // TESTING ONLY!
    pub fn _run_all(&mut self) {
        for source in &mut self.sources {
            let src = &mut **source;
            src.run();
        }
    }
}
