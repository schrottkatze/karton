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

## TODOs:

- [x] removed light mode

- [x] Rebrand
	- [x] New name and logo
	- [x] New README
	- [ ] installation guides
	- [ ] Website
	- [ ] Official central instance
	- [ ] Donation thing?

- [ ] Distribution
	- [ ] nixpkgs
	- [x] crates.io
	- [ ] Docker
	- [ ] Various other distribution specific repositories?

- [ ] Quality
	- [ ] Tests
	- [x] New theme
	- [ ] Proper database (_seriously, json isn't a database_)
	- [ ] Proper auth and permissions (_so a single user can also use it and send links_)
		- [ ] multi-user
	- [x] Click logo/name to get to root

- [ ] Customizability
	- [ ] Customizable keys (_so you can make fixed pastas_)
		- [x] Customizable wordset 
	- [ ] Non-env/args configurations
	- [ ] further endpoint configuration
		- [x] customizable `pasta`, `url` and `raw` endpoints
		- [ ] simplified media embed endpoints (/file/$id or /embed/$id by default? maybe with compression?)
		- [ ] disable remove
		- [ ] make root page a redirect and root based redirect
	- [ ] easy to customize logo, icon etc.
	- [ ] simplified themeing (only colors etc)

- [ ] Features
	- [ ] encrypted pastas
	- [ ] image embeds
		- [x] in pasta view
		- [ ] easy to copy image embed url
	- [ ] Markdown pastas



## Contact

This fork of MicroBin was created by [Schrottkatze](https://schrottkatze.de). 

Join [the matrix room](https://matrix.to/#/#s10e-microbin:matrix.org) to chat!

Contact me via e-mail at [contact@schrottkatze.de](mailto:contact@schrottkatze.de).
