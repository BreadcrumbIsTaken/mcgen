## Commands

|command|description|
|  :-:  |    :-:    |
|[add](https://github.com/BreadcrumbIsTaken/mcgen/blob/main/DOCS.md#add)|Add additional plugins to an already existing server|
|[config](https://github.com/BreadcrumbIsTaken/mcgen/blob/main/DOCS.md#config)|Opens the config in your computer's default text editor|
|[gen](https://github.com/BreadcrumbIsTaken/mcgen/blob/main/DOCS.md#gen)|Generate a new server|
|[update](https://github.com/BreadcrumbIsTaken/mcgen/blob/main/DOCS.md#update)|Updates plugins or server/bungeecord versions|

### `add`

#### Subcommands
|subcommand|description|
|   :-:    |    :-:    |
|[bungeecord](https://github.com/BreadcrumbIsTaken/mcgen/blob/main/DOCS.md#bungeecord)|Add a BungeeCord server.|
|[paper](https://github.com/BreadcrumbIsTaken/mcgen/blob/main/DOCS.md#paper)|Add a Paper server.|
|[plugin](https://github.com/BreadcrumbIsTaken/mcgen/blob/main/DOCS.md#plugin)|Add a plugin. Must have a Jenkins API|
|[startfile](https://github.com/BreadcrumbIsTaken/mcgen/blob/main/DOCS.md#start-file)|Add a start file.|

##### `bungeecord`

###### Arguments
|argument|description|example|
|   :-:  |    :-:    |  :-:  |
|DIRECTORY|The directory to add BungeeCord to.|myserver|

###### Flags
|flags|description|example usage|long name|
| :-: |    :-:    |     :-:     |   :-:   |
|-a   |Choose whether or not to use [Aikar's Flags](https://aikar.co/mcflags.html)|`mcgen add bungeecord myserver -a`|--aikars-flags|
|-j   |When adding BungeeCord, if you only want to add the jar and not have all the other plugins installed, use this flag|`mcgen add bungeecord myserver -j`|--jar-only|
|-n   |By using this option mcgen will not generate the start scripts. By not using this flag mcgen continues its default behavior and will generate the start scripts.|`mcgen add bungeecord myserver -n`|--no-start-scripts|
|-o   |If BungeeCord or any other generated files already exist, overwrite them instead of throwing an error.|`mcgen add bungeecord myserver -o`|--overwrite|

###### Example
```
mcgen add bungeecord . -a
```

##### `paper`

###### Arguments
|argument|description|example|
|   :-:  |    :-:    |  :-:  |
|DIR|The directory to add Paper to|`mcgen add paper myserver`|

###### Flags
|flags|description|example usage|long name|
| :-: |    :-:    |     :-:     |   :-:   |
|-a   |Choose whether or not to use [Aikar's Flags](https://aikar.co/mcflags.html).|`mcgen add paper myserver -a`|--aikars-flags|
|-j   |When adding Paper, if you only want to add the jar and not have all the other plugins installed, use this flag|`mcgen add paper myserver -j`|--jar-only|
|-e   |Accept to Minecraft's EULA. Wilmcgen add paper myservermcgen gen myserver -e`|--accept-eula|
|-n   |By using this option mcgen will not generate the start scripts. By not using this flag mcgen continues its default behavior and will generate the start scripts.|`mcgen add paper myserver -n`|--no-start-scripts|
|-o   |If Paper or any other generated files already exist, overwrite them instead of throwing an error.|`mcgen add paper myserver -o`|--overwrite|

###### Example
```
mcgen add paper . -n -e
```

##### `plugin`

###### Arguments
|argument|description|example|
|   :-:  |    :-:    |  :-:  |
|DIRECTORY|The directory to add the plugin to.|`mcgen add plugin myserver`|
|NAME|Name of the plugin.|`mcgen add plugin myserver Denizen`|
|URL|URL to download plugin. MUST HAVE A JENKINS API!|`mcgen add plugin myserver Denizen https://ci.citizensnpcs.co/job/Denizen`|

##### Flags
|flags|description|example usage|long name|
| :-: |    :-:    |     :-:     |   :-:   |
|-o   |If the plugin already exists, overwrite them instead of throwing an error|`mcgen add plugin myserver Denizen https://ci.citizensnpcs.co/job/Denizen -o`|--overwrite|

###### Example
```
mcgen add plugin myserver Denizen https://ci.citizensnpcs.co/job/Denizen
```

##### `startfile`

###### Arguments
|argument|description|example|
|   :-:  |    :-:    |  :-:  |
|DIRECTORY|The directory to add the start files to.|`mcgen add plugin myserver`|

##### Flags
|flags|description|example usage|long name|
| :-: |    :-:    |     :-:     |   :-:   |
|-a   |Choose whether or not to use [Aikar's Flags](https://aikar.co/mcflags.html).|`mcgen add startfile myserver -a`|--aikars-flags|
|-o   |If the start files already exist, overwrite them instead of throwing an error.|`mcgen add startfile myserver -o`|--overwrite|

### `config`

##### Flags
|flags|description|example usage|long name|
| :-: |    :-:    |     :-:     |   :-:   |
|-r   |Will delete the contents of the current config and create a new one with the default values and then open it.|`mcgen config -r`|--regenerate|

##### Example
```
mcgen config
```

### `gen`

##### Arguments
|argument|description|example|
|   :-:  |    :-:    |  :-:  |
|DIR|The name of the directory to place contents of the Minecraft server|`mcgen gen myserver`|

##### Flags
|flags|description|example usage|long name|
| :-: |    :-:    |     :-:     |   :-:   |
|-a   |Choose whether or not to use [Aikar's Flags](https://aikar.co/mcflags.html).|`mcgen gen myserver -a`|--aikars-flags|
|-b   |Choose whether or not to have a BungeeCord network.|`mcgen gen myserver -b`|--bungeecord|
|-e   |Accept to Minecraft's EULA. Will create the file automatically with `eula` set to `true`. By using this option, you agree to accept the [EULA](https://aka.ms/MinecraftEULA)|`mcgen gen myserver -e`|--accept-eula|
|-n   |By using this option mcgen will not generate the start scripts. By not using this flag mcgen continues its default behavior and will generate the start scripts.|`mcgen gen myserver -n`|--no-start-scripts|

##### Example
```
mcgen gen myserver -b -a
```

### `update`

##### Arguments
|argument|description|example|
|   :-:  |    :-:    |  :-:  |
|DIRECTORIES|A comma-then-space seperated list of directories to update the plugins/servers. Will look for the mcgen.txt file in the current directory if not set|

##### Flags
|flags|description|example usage|long name|
| :-: |    :-:    |     :-:     |   :-:   |
|-c   |Will check for any updates, but will not install them|`mcgen update myserver -c`|--check|

##### Example
```
mcgen update myserver, bungeecord
```
