# A psuedonymous blog

## Deployment Process
An initial, basic deployment involved just scp the build artifacts, frontend and backend to the Pi.
However, not sure yet how to just run these and how it serves the site.
A question I have is what is the difference between the frontend build files in `dist/` and the backend
build files in `target/`.

Intuitively, backend build files should build the actix-web files and produce an executable binary,
that, when run, should run `main()`, which essentially connects to the postgres database then starts up
the http webserver, binding it to 0.0.0.0 and port 80 (or 443).

The frontend build contains the index.html files and the wasm files with JS wrappers? Something like that.
Yes, frontend build has js, wasm, and css files. Css is just stylesheet. Wasm binary contains the compiled
Rust -> wasm code, which contains the Yew application logic. JS, as I understand, are wrappers around the wasm
functions that allow the browser to then run WASM. JS is needed to instantiate the wasm module, amongst other
things.

The backend server then serves the static index.html file, which is then loaded upon access to root url, and
then from there the JS scripts take over, and Yew framework is run.

### scp entire source code
Currently, just doing this, and then building artifacts directly on pi. This has a problem, since it is quite
slow to compile each time:

- Look into using docker


### Problems
If I just scp the artifacts over, then when I make source code changes, I need to rebuild and then scp again.
If I scp source code, need to do the same thing, rebuild, which is more time consuming. Not going to do the
source code route in long term, since I don't want to do remote development, and work on the slow Pi.

So going the artifacts-only route is better. But like I said, need to rebuild source and scp each time. 
Furthermore, are there certain dependencies I must download, or can any system (as long as right architecture)
just run the binary executable? **Look into this**.

So Docker might have the following advantages:
1. Dependencies - with a docker image, can just have the right dependencies on any platform, no need to worry.
2. Rebuild source and scp each time - not sure if docker solves this...

## Current Strategy
To just get it up and running asap:
- I've got the source code on Pi, and I build on pi
- Use those built artifacts and run the executable, which starts the webserver
- With right portforwarding and everything, this should work (?)

## Next steps
If that works, and I can access my website just by typing my router's ip into the browser:
1. Look into systemd - keep webserver running at all times
2. Look into nginx - setting reverse proxy up, and SSL
3. Then use systemd to startup my nginx and webserver

## Second Iteration
Once site is up and running, then get a domain name:
1. Purchase domain, see if I need SSL or how that works (does nginx provide it)
2. Configure domain so it points to the IP address of my router

## Third Iteration: deployment process streamlining
At this point, site should be up and running reliably. People on the outside interweb should
be able to access my site via a domain name: victo.rs or something. At this point, I'll look into making the
deployment process more streamlined.
1. Look into docker - if I need it, and then if I don't need systemd?
