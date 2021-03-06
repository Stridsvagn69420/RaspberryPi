# RaspberryPi
Source Code for my Raspberry Pi 4's LAN Web Server. It also includes hosting an instance of [Shigure](https://github.com/Stridsvagn69420/Shigure) and [Cyrkensia](https://github.com/Stridsvagn69420/Cyrkensia).

# Developing
## Commands
* `npm run release`: Compiles the Svelte GUI and the Actix-Web Server for release
* `npm run build`: Compiles the Svelte GUI and the Actix-Web Server
* `npm run dev`: Starts a dev environment for developing the Server
* `npm start`: Basically just starts the server with its GUI as is.
* `npm run check`: Checks the Svelte GUI for any errors

## dev-config.json
If you don't use Linux (how dare you?!) or don't want to create a folder, that you will eventually forget about, just to test this, you can create a local `dev-config.json`. This file is also needed if you just want to develop on this:
```json
{
    "server": {
        "path": "/home/joe/Code/RaspberryPi/www/public",
        "bindaddr": "127.0.0.1:9000",
        "socket": false
    },
    "cyrkensia": {
        "path": "/home/joe/Music",
        "uuid": "ba046cd1-2c09-480d-ac67-bc422ad540d7",
        "name": "Raspberry Pi",
        "hosticon": "raspberrypi",
        "htpasswd": "dev.htpasswd"
    },
    "shigure": {
        "path": "/home/joe/Code/Shigure/build/web"
    }
}
```
You can find an explaination for every field in the docs generated from the Rust code.