# MServer

Serve your markdown files to the internet.

## Build it yourself.

1. Install the rust toolchain.
2. Build using cargo.
3. Run the executable.

The executable will create a default folder with the configurations that is needed to successfully run this project. The
folder should also contain the markdown files that will be served by the server.

### Todo

- [ ] Add HTTPS support.
- [x] Extract hardcoded configuration to a configuration file.
- [ ] Add an PKGBUILD script for arch based distros.
- [ ] Update the configuration file if there missing fields.
- [ ] Finish the routing
- [ ] Use the current path as working directory and skip the whole default configuration path.
- [ ] Make it possible to inject CSS, Javascript.
- [ ] Read title from the configuration and remove the hardcoded title.
- [ ] Make the configuration available at runtime without IO in the middle.
