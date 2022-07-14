# mcgen

## Generate a Minecraft server in seconds!

Generates a server with [Paper](https://papermc.io) and optionally [BungeeCord](https://www.spigotmc.org/threads/1-8-1-15-bungeecord.392/). This tool will auto generate a simple start script (start.sh/start.bat depending on what OS you use) or one with [Aikar's Flags](https://aikar.co/mcflags.html).

## How to use!

Simply install it using:

```
cargo install mcgen
```

then you just have to use a super easy command:

```
mcgen gen your_servers_name_here
```

## Commands

|command|description|
|  :-:  |    :-:    |
|[add](https://github.com/BreadcrumbIsTaken/mcgen#add)|Add additional plugins to an already existing server|
|[config](https://github.com/BreadcrumbIsTaken/mcgen#config)|Opens the config in your computer's default text editor|
|[gen](https://github.com/BreadcrumbIsTaken/mcgen#gen)|Generate a new server|
|[update](https://github.com/BreadcrumbIsTaken/mcgen#update)|Updates plugins or server/bungeecord versions|

### `add`

##### Arguments
|argument|description|example|
|   :-:  |    :-:    |  :-:  |
|DIR|The directory to add the plugin to|myserver|
|NAME|Name of the plugin|Denizen|
|URL|Url to download the plugin. MUST HAVE A JENKINS API! Leave out the trailing slash.|https://ci.citizensnpcs.co/job/Denizen|

##### Example
```
mcgen add myserver Denizen https://ci.citizensnpcs.co/job/Denizen
```

### `config`

##### Example
```
mcgen config
```

### `gen`

##### Arguments
|argument|description|example|
|   :-:  |    :-:    |  :-:  |
|DIR|The name of the directory to place contents of the Minecraft server|

##### Flags
|flags|description|example usage|long name|
| :-: |    :-:    |     :-:     |   :-:   |
|-a   | Choose whether or not to use [Aikar's Flags](https://aikar.co/mcflags.html).|`mcgen gen myserver -a`|--aikars-flags|
|-b   | Choose whether or not to have a BungeeCord network.|`mcgen gen myserver -b`|--bungeecord|

##### Example
```
mcgen gen myserver -b -a
```

###### License:

    MIT License

    Copyright (c) 2022 Breadcrumb (https://breadcrumb.fun)

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

###### 🌾🌾🌾
