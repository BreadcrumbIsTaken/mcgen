pub struct GenSettings {
    directory: Option<String>,
    bungeecord: bool,
    aikars_flags: bool,
    accept_eula: bool,
    dont_generate_start_scripts: bool,
    version: Option<String>,
}

impl GenSettings {
    pub fn new() -> Self {
        Self {
            directory: None,
            bungeecord: false,
            aikars_flags: false,
            accept_eula: false,
            dont_generate_start_scripts: false,
            version: None,
        }
    }

    pub fn set_directory(&mut self, dir: String) {
        self.directory = Some(dir);
    }

    pub fn set_bungeecord(&mut self, bungeecord: bool) {
        self.bungeecord = bungeecord;
    }

    pub fn set_aikars_flags(&mut self, aikars_flags: bool) {
        self.bungeecord = aikars_flags;
    }

    pub fn set_accept_eula(&mut self, accept_eula: bool) {
        self.accept_eula = accept_eula;
    }

    pub fn set_dont_gen_start_scripts(&mut self, gen_start_scripts: bool) {
        self.dont_generate_start_scripts = gen_start_scripts;
    } 

    pub fn set_version(&mut self, version: Option<String>) {
        self.version = version;
    }
}
