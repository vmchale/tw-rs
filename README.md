# Command Line Interface Tweeter (rust version)

![Displaying a user's timeline in a terminal.](https://raw.githubusercontent.com/vmchale/clit-rs/master/twitter-rust-screenshot.png)

## Config

Generate a token to authorize access to your twitter account by following the guide [here](https://dev.twitter.com/oauth/overview/application-owner-access-tokens)

Then place your API keys and OAuth tokens in a file (default is `~/.cred`), separated by a line break:

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
from the releases page [here](https://github.com/vmchale/clit-rs/releases).

To build from source, install [cargo](https://www.rustup.rs/) via rustup; on unix systems this is as simple as

```
curl https://sh.rustup.rs -sSf | sh
```

Then type `cargo install clit-rs`.

## Usage

### View Profiles

To get your profile, simply type:

```
tw user
```

To view a user's profile, type e.g.

```
tw user pinepapplesmear
```

If you have any problems along the way:

```
tw --help
```

### Sending tweets

```
tw send "YOUR_TWEET_TEXT"
```

### Viewing your timeline

You can also use

```
tw view
```

to view your own timeline.

## Library

A library is included. It's fairly easy to use once you have the credentials set up, with three functions: one to post a status, one to get your timeline, and one to get a user profile.

### Haskell

There is a haskell version of this, with a binary and a library, avaiable [here](https://github.com/vmchale/command-line-tweeter).
