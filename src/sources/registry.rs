/// The Registry containing the sources' main functions that can be called.
pub struct SourceRegistry {
    plugins: Vec<Box<dyn FnMut()>>,
}

impl SourceRegistry {
    pub fn new() -> Self {
        Self { plugins: vec![] }
    }

    pub fn add_source<F>(&mut self, f: F)
    where
        F: Fn() + 'static,
    {
        self.plugins.push(Box::new(f));
    }

    // TESTING ONLY!
    fn _run_all(&mut self) {
        for plugin in &mut self.plugins {
            let var = &mut **plugin;
            var();
        }
    }
}
