# MicroBin

This is a fork of [MicroBin](https://github.com/szabodanika/microbin) intends to fix some bugs and add some features to MicroBin.

I deliberately removed docker support for now. 

Please do not ask for help in the issues of the original MicroBin repository, I'd be happy to help you with any problems here though. 

## TODOs:

- [ ] Rebrand
	- [ ] New name and logo
	- [ ] New README
	- [ ] Website
	- [ ] Official central instance
	- [ ] Donation thing?

- [ ] Distribution
	- [ ] nixpkgs
	- [ ] crates.io
	- [ ] Docker
	- [ ] Various other distribution specific repositories?

- [ ] Quality
	- [ ] Tests
	- [ ] Proper design stuff

- [ ] Proper database (_seriously, json isn't a database_)
- [ ] Configurable endpoints (_so a url shortener isn't lengthened by /url/_)
	- [ ] make root page a redirect and root based redirect
- [ ] Proper auth and permissions (_so a single user can also use it and send links_)
	- [ ] multi-user
- [ ] Customizable keys (_so you can make fixed pastas_)
	- [x] Customizable wordset 
- [ ] Non-env/args configurations
- [ ] encrypted pastas
- [ ] image embeds
	- [x] in pasta view
	- [ ] easy to copy image embed url
- [ ] Markdown pastas
- [ ] further endpoint configuration
	- [x] customizable `pasta`, `url` and `raw` endpoints
	- [ ] disable remove
- [x] Click logo/name to get to root
- [ ] Move frontend interactive code to rust as well

- [x] removed light mode


## Contact

This fork of MicroBin was created by [Schrottkatze](https://schrottkatze.de). 

Join [the matrix room](https://matrix.to/#/#s10e-microbin:matrix.org) to chat!
