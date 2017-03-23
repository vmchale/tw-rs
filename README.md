# Command Line Interface Tweeter (rust version)

[![Build Status](https://travis-ci.org/vmchale/clit-rs.svg?branch=master)](https://travis-ci.org/vmchale/clit-rs)

Screenshot in alacritty:

![Displaying a user's timeline in a terminal.](https://raw.githubusercontent.com/vmchale/clit-rs/master/twitter-rust-screenshot.png)

The rust version of this tool is somewhat faster than its [haskell
counterpart](https://github.com/vmchale/command-line-tweeter), but the haskell
one has a couple more features.

Reasons to use tw-rs:
  - Faster than other tools ([t](https://github.com/sferik/t),
  ,[tweet-hs](https://github.com/vmchale/command-line-tweeter)
  ,[oysttyer](https://github.com/oysttyer/oysttyer))
  - Lightweight (5MB, low CPU/memory usage)
  - Unobtrusive
  - Support for colored output. 
  - Can be used in scripts
  - You know rust and like being able to extend your tools. 
  - You want something that can be called from
    [vim](https://github.com/vmchale/vim-twitter)
  - You want a twitter library for rust. 
  - BSD3 licensed 

Reasons not to use tw-rs:
  - Several features are still in development
  - Fewer features than [rainbowstream](https://github.com/DTVD/rainbowstream),
    [t](https://github.com/sferik/t), or [oysttyer](https://github.com/oysttyer/oysttyer)
  - You want to extend your tools in [haskell](https://github.com/vmchale/command-line-tweeter)
  - You want "twitter in a terminal" that [rainbowtools](https://github.com/DTVD/rainbowstream)
    or [oysttyer](https://github.com/oysttyer/oysttyer) provides. 

## Config

Generate a token to authorize access to your twitter account by following the guide [here](https://dev.twitter.com/oauth/overview/application-owner-access-tokens)

Then place your API keys and OAuth tokens in a file (default is `$HOME/.cred`), separated by a line break:

```
api-key: API_KEY_HERE
api-sec: API_SECRET_HERE
tok: OAUTH_TOKEN_HERE
tok-sec: TOKEN_SECRET_HERE
```

Note that the labels must in the correct order, and separated from the keys with
whitespace. 

## Installation

If you're on Linux/Windows the best way is probably to download the binaries
from the releases page [here](https://github.com/vmchale/tw-rs/releases).

Otherwise, you'll have to build from source. To build from source, install 
[cargo](https://www.rustup.rs/) via rustup; on unix systems this is as simple as

```
curl https://sh.rustup.rs -sSf | sh
```

Then type `cargo install tw-rs`.

## Use

### View Profiles

To get your profile, simply type:

```bash
$ tw user
```

To view a user's profile, type e.g.

```bash
$ tw user pinepapplesmear
```

If you have any problems along the way:

```bash
$ tw help
```

### Sending tweets

```bash
$ tw send "YOUR_TWEET_TEXT"
```

### Viewing your timeline

You can also use

```bash
$ tw view
```

to view your own timeline.

### Features

  - [x] tweet
  - [x] view timeline
  - [x] view user profiles
  - [x] output with id of tweet
  - [x] delete tweet
  - [x] retweet
  - [x] reply to tweet
  - [x] fav/unfav tweets
  - [ ] follow/unfollow users
  - [ ] fetch mentions
  - [ ] block accounts
  - [ ] tweet an image
  - [ ] display quoted tweets alongside

### Speed

In the `bash` directory you will find a script that times tw-rs, tweet-hs,
oysttyer, and t fetching tweets. On linux, it consistently places `tw` as
the fastest.

### Comparison to other command-line clients

| Tool | Language | Color output | Interactive | Vim plugin support | Scriptable |
| ---- | -------- | ------------ | ----------- | ------------------ | ---------- |
| tw | Rust | x |   | x | x |
| rainbowstream | Python | x | x |  |  |
| oysttyer | Perl |  | x |  | ½ |
| tweet-hs | Haskell | x |  | x | x |
| t | Ruby | ½ |  |  | x |

#### Screenshots in alacritty

##### tw

![Displaying a user's timeline in a terminal.](https://raw.githubusercontent.com/vmchale/clit-rs/master/screenshots/rusttw.png)

##### t

![Displaying a user's timeline in a terminal.](https://raw.githubusercontent.com/vmchale/clit-rs/master/screenshots/rubyt.png)

##### rainbowstream

![Displaying a user's timeline in a terminal.](https://raw.githubusercontent.com/vmchale/clit-rs/master/screenshots/pythonrainbowstream.png)

##### tweet

![Displaying a user's timeline in a terminal.](https://raw.githubusercontent.com/vmchale/clit-rs/master/screenshots/haskelltweet.png)

##### oysttyer

![Displaying a user's timeline in a terminal.](https://raw.githubusercontent.com/vmchale/clit-rs/master/screenshots/perloysttyer.png)

## Coloring

tw-rs respects the [CLICOLOR behavior](http://bixense.com/clicolors/) defined here.
If you wish to disable colorization, 

```bash
 $ export CLICOLOR=0
```

## Library

A library is included. It's fairly easy to use once you have the credentials set up, with three functions: one to post a status, one to get your timeline, and one to get a user profile.

### Haskell

There is a haskell version of this, with a binary and a library, available [here](https://github.com/vmchale/command-line-tweeter). The haskell library is more complete.
