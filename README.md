# Karton

A small, rusty pastebin with URL shortener functionality.

The github repository is a mirror of [this gitlab repository](https://gitlab.com/obsidianical/microbin).

This is a fork of [MicroBin](https://github.com/szabodanika/microbin).

## Features 

- Animal names (by default) or custom namefiles instead of just hashes (though hashes are an option too!)
- File and image uploads
- raw text serving
- URL shortening
- QR codes
- Listing and removing pastas (though currently everyone can do that)
- Expiration times
- Editable pastas
- Syntax highlighting
- Styling via [water.css](https://github.com/kognise/water.css)
- Customizable endpoints

## Roadmap

This is only a rough time guide for what to get done by which version, nothing fixed.

### v2.0

- [x] Removed light mode
- [x] Rebrand
	- [x] New name and logo
	- [x] New README
	- [x] New theme
- [x] Distribution on crates.io
- [x] Distribution on Docker hub
- [x] Image embedding in pasta page
- [x] Custom names file
- [x] Click logo/name to get to root

### v2.1

- [ ] Installation guides 
- [ ] Copying embed urls more easily (some button for that)
- [ ] Api for requesting customized url info
- [ ] Config file support 
- [ ] Markdown pastas

### v2.2

- [ ] Proper docs
- [ ] Improved rustdoc
- [ ] Non-web client library 
- [ ] CLI client
- [ ] Easier customization of instance names and logos
- [ ] Easier basic themeing
	- [ ] Colors
	- [ ] Corner rounding

### v2.3

- [ ] Encrypted pastas

### v3.0

- [ ] Deprecate JSON db, replace with SQLite
- [ ] Rework internal data structures
	- [ ] Map custom keys to ids
	- [ ] Make hash ids, names and custom keys usable at once
- [ ] Users
	- [ ] Reporting pastas
	- [ ] Admin panel
- [ ] Improve remove api
- [ ] Status/health/info improvements
	- [ ] Storage thats left
	- [ ] Db status
	- [ ] Pastas on the instance
	- [ ] Users
	- [ ] Errors

### future

- [ ] Federation
	- [ ] Requesting pastas from other instances
	- [ ] When cli is complete, set default instance to route requests over
	- [ ] Fine grained permissions
- [ ] Postgresql db support
- [ ] Pasta comments?

### not related to features, therefor not versioned

- [ ] Official central instance
- [ ] Donation setup?
- [ ] Distribution on nixpkgs
- [ ] Website

## Contact

This fork of MicroBin was created by [Schrottkatze](https://schrottkatze.de). 

Join [the matrix room](https://matrix.to/#/#s10e-microbin:matrix.org) to chat!

Contact me via e-mail at [contact@schrottkatze.de](mailto:contact@schrottkatze.de).
