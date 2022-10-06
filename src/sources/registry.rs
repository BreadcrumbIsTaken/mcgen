/// Source trait for Sources.
pub trait Source {
    fn get_label(&self) -> &String;
    fn download(&self);
    fn set_mcgen_file_data(&mut self);
    fn run(&self);
}

/// The Registry containing the sources' main functions that can be called.
pub struct SourceRegistry {
    sources: Vec<Box<dyn Source>>,
}

impl SourceRegistry {
    /// Create a new instance of the [`SourceRegistry`]
    pub fn new() -> Self {
        Self { sources: vec![] }
    }

    /// Add a new source to the [`SourceRegistry`].
    pub fn register_source<S>(&mut self, source: S)
    where
        S: Source + 'static,
    {
        self.sources.push(Box::new(source));
    }

    /// Run the source.
    pub fn run_source(&self, label: String) {
        // Search for the source based on the label.
        for source in &self.sources {
            let src = &(**source);
            if *src.get_label() == label {
                src.run();
            }
        }
    }

    // TESTING ONLY!
    pub fn _run_all(&mut self) {
        for source in &mut self.sources {
            let src = &mut **source;
            src.run();
        }
    }
}
