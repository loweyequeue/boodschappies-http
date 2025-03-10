# boodschappies-http

A HTTP server that is intended to be used with Boodschappies the iOS app (currently unreleased).

The iOS app is a self-hosted free shopping/grocery list app that allows multiple users to work on the list without costs or ads.

## Features

- Keeping track of grocery lists
- Easily shared by having family connect to the same server
- Live updates from other users


## Installation

**NOTE: a more straight forward way of installation may be provided once the app is released.**

Install the server to your system like this:

```bash
git clone https://github.com/loweyequeue/boodschappies-http.git
cd boodschappies-http
cargo install --path .
```

Then create a SYSTEMD service (or any init/service system file) to run the server.

*NOTE: the 'database' is created in the working directory of the server as database.json*

Then make sure you add BOODSCHAPPIES_HMAC_KEY to your env.  
Or as alternative add it to the Settings.toml (UNSAFE)

Finally run your service

RECOMMENDED: run caddy/nginx/whatever to use HTTPS


## Configuration

run the server once to generate the default configuration (Settings.toml)

**NOTE: DO NOT FORGET THE CHANGE THE HMAC KEY**

all configuration can be overwritten with environment variables, i.e:

`BOODSCHAPPIES_HMAC_KEY="mysupersecretkey" boodschappies-http`

## TODO
- improve stability (server will currently crash when errors happen)
- add logging system
- Cleanup/move ''database''
