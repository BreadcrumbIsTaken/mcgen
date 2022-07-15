## Commands

|command|description|
|  :-:  |    :-:    |
|[add](https://github.com/BreadcrumbIsTaken/mcgen/blob/main/DOCS.md#add)|Add additional plugins to an already existing server|
|[config](https://github.com/BreadcrumbIsTaken/mcgen/blob/main/DOCS.md#config)|Opens the config in your computer's default text editor|
|[gen](https://github.com/BreadcrumbIsTaken/mcgen/blob/main/DOCS.md#gen)|Generate a new server|
|[update](https://github.com/BreadcrumbIsTaken/mcgen/blob/main/DOCS.md#update)|Updates plugins or server/bungeecord versions|

### `add`

##### Arguments
|argument|description|example|
|   :-:  |    :-:    |  :-:  |
|DIRECTORY|The directory to add the plugin to|myserver|
|NAME|Name of the plugin|Denizen|
|URL|Url to download the plugin. MUST HAVE A JENKINS API! Leave out the trailing slash.|https://ci.citizensnpcs.co/job/Denizen|

##### Example
```
mcgen add myserver Denizen https://ci.citizensnpcs.co/job/Denizen
```

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
