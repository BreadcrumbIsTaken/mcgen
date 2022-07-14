pub const DEFAULT_CONFIG_STRING: &str = r#"# These plugins will be installed whenever you make a new server with the "gen" command.
default_plugins:
    paper_plugins:
        # Name of the plugin: Link to API (Only works for Jenkins's JSON API)
        # Example:
        #- Denizen: https://ci.citizensnpcs.co/job/Denizen/lastStableBuild/api/json
    bungeecord_plugins:
        #- DepenizenBungee: https://ci.citizensnpcs.co/job/DepenizenBungee/lastStableBuild/api/json
"#;
