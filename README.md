# GitHub Releases (ghr)

**Ghr** is a small command-line utility for tracking new releases of your favourite OSS from GitHub - think of it as an attempt at making `apt` with GitHub as the backend.

## Installation and Usage

You can choose to download the source and build it with the stable Rust tool-chain, by smashing the following into a terminal:

```sh
cargo build --release
```

Or, you can download a binary from the Releases page.

Either way, ensure the executable is made available via your `PATH` and you'll be good to get familiar with the utility via:

```sh
ghr -h
```

(There's not a lot to learn, I promise)

## Basic Features

You can `install` a repo, which simply downloads the zipball of a GitHub repository's latest release.

Ghr will keep a note of the latest downloaded zipball, so that when you run `update`, it will only ever download newer versions of installed releases.

Naturally, should you decide that you no longer like the product of a repository, you can `remove` it from ghr and it'll no longer be tracked.

## Slightly Sexy Features

Yes, I'm well aware that we're just downloading a zip file and making sure that we have the latest zip file when we choose to update.

What if I told you that you can run scripts upon install, update and removal of a repository's release?

Whether it be using `sed` for some super string replacement or simply `echo`ing your favourite Post Malone lyrics - You get complete control over the compilation and installation of the software downloaded from GitHub.

I would recommend either having the tools required to build downloaded releases installed on your machine, or available via a docker container - this way you can build the extracted source code and then move the output binary(s) to your preferred location on your file system for use.

## Things To Do

If people start using this, I imagine the Issues page will probably start filling up - so check there if you'd like to get involved.

PR's are very welcome - please make sure that they relate to a discussion on the Issues page.

The initial thing I'd like to focus on is optimisation of the code. I'm very new to Rust and I'm pretty certain that I've broken a few standards, or at least missed a few tricks.

If any Rustaceans or code-connoisseurs are about, I'd love your input on how my code can improve (Please be as descriptive as possible, maybe share examples where you can)