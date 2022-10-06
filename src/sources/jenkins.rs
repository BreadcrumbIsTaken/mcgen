//! # Jenkins Source
//! Fetches data from a JenkinsAPI and downloads the JAR file.

use super::registry::Source;

pub struct JenkinsSource {
    label: String,
}

impl JenkinsSource {
    pub fn new(label: String) -> Self {
        Self { label }
    }
}

impl Source for JenkinsSource {
    fn get_label(&self) -> &String {
        &self.label
    }

    fn set_mcgen_file_data(&mut self) {
        todo!()
    }

    fn download(&self) {
        println!("downloading...");
    }

    fn run(&self) {
        self.download();
    }
}
