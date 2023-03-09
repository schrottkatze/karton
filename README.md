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

## Installation guide

Karton is available on [Docker hub](https://hub.docker.com/r/schrottkatze/karton), [crates.io](https://crates.io/crates/karton) and using the nix flake.

The only "officially supported" (I will actively debug and search for the problem) method is the last one using nix flakes.

### Installation via the nix flake

Add the repository to your inputs.

```nix
	karton.url = "git+https://gitlab.com/obsidianical/microbin.git";
```

```nix
# microbin.nix
{ inputs, config, pkgs, ... }:
{
  environment.systemPackages = [ inputs.karton.defaultPackage."x86_64-linux" ];
  systemd.services.karton = {
    after = [ "network.target" ];
    wantedBy = [ "multi-user.target" ];
    environment = {
	  # set environment variables to configure karton
      KARTON_HASH_IDS = "";
      KARTON_EDITABLE = "";
      KARTON_PRIVATE = "";
      KARTON_HIGHLIGHTSYNTAX = "";
      # adjust this to your domain
      KARTON_PUBLIC_PATH = "https://example.org";
      KARTON_QR = "";
      # configure endpoints to be shorter
      KARTON_URL_EP = "u";
      KARTON_RAW_EP = "r";
      KARTON_PASTA_EP = "p";
    };
    script = "${inputs.karton.defaultPackage."x86_64-linux"}/bin/karton";
    # register a simple systemd service
    serviceConfig = {
      Type = "simple";
      RootDirectory="/";
      WorkingDirectory = "/karton";
    };
  };
}
```

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
	 - [x] nix flake
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
