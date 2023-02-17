# MicroBin

MicroBin is a super tiny, feature rich, configurable, self-contained and self-hosted paste bin web application. It is very easy to set up and use, and will only require a few megabytes of memory and disk storage. It takes only a couple minutes to set it up, why not give it a try now?

## Note about this fork

I've forked MicroBin due to apparent neglect from the maintainer, and to fix various issues and annoyances to me personally.

### TODOs:

- [ ] Proper database (_seriously, json isn't a database_)
- [ ] Configurable endpoints (_so a url shortener isn't lengthened by /url/_)
- [ ] Proper auth and permissions (_so a single user can also use it and send links_)
- [ ] Customizable keys (_so you can make fixed pastas_)
	- [ ] Customizable wordset and length for the names
- [ ] Non-env/args configurations

## Features

- Is very small
- Entirely self-contained executable, MicroBin is a single file!
- Animal names instead of random numbers for pasta identifiers (64 animals)
- File uploads (eg. `server.com/file/pig-dog-cat`)
- Raw text serving (eg. `server.com/raw/pig-dog-cat`)
- URL shortening and redirection
- QR code support
- Very simple database (JSON + files) for portability, easy backups and integration
- Listing and manually removing pastas (`/pastalist`)
- Private and public, editable and final, automatically and never expiring pastas
- Syntax highlighting
- Automatic dark mode and custom styling support with very little CSS and only vanilla JS (see [`water.css`](https://github.com/kognise/water.css))
- Most of the above can be toggled on and off!

## What is a "pasta" anyway?

In MicroBin, a pasta can be:

- A text that you want to paste from one machine to another, eg. some code,
- A file that you want to share, eg. a video that is too large for Discord, a zip with a code project in it or an image,
- A URL redirect.

## When is MicroBin useful?

You can use MicroBin:

- As a URL shortener/redirect service,
- To send long texts to other people,
- To send large files to other people,
- To serve content on the web, eg. configuration files for testing, images, or any other file content using the Raw functionality,
- To move files between your desktop and a server you access from the console,
- As a "postbox" service where people can upload their files or texts, but they cannot see or remove what others sent you - just disable the pastalist page
- To take notes! Simply create an editable pasta.

...and many other things, why not get creative?

MicroBin and MicroBin.eu are available under the [BSD 3-Clause License](LICENSE).

© Dániel Szabó 2022
