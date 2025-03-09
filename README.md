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

Finally run your service

OPTIONAL: run caddy/nginx/whatever to expose the server (it runs on 127.0.0.1:30301 by default)



## TODO
- configurable host/port
- improve stability (server will currently crash when errors happen)
- add logging system
- Some form of authentication or a PSK/HMAC to make sure no strangers can find your server and use it
