# Freezer
![License](https://img.shields.io/badge/license-MIT-blue.svg)

A command line tool to manage and publish a personal digest of articles. The core premise is keeping a list of RSS and Atom feeds, and then publishing any new articles in the last week to email. 

## Caveats
Currently, it only works on Linux, and relies on a local config stored at `~/.config/freezer/freezer.toml`. For the programme to work the config needs the following minimal set up:

```toml
name = "<any name>"
email = "<email you would like to receive the digest at>"
time_period_hours = 168
feeds = []
```

## Installing Freezer
You will need to clone this project and then you can install it using `cargo install --path <path to repo>`.

## Using the tool
There is a helpful dialog which will pop up if you just run the tool `freezer`. 

## ToDos
The basic functionality is there, and I have been using it for a while now, but there are definitely still a lot of todos. 

- [x] Remove the need for an email from the publish command
- [x] Remove feed flag from add/remove, just have the feed
- [x] Convert feeds to HashSet
- [ ] Connect to the application from mobile
- [ ] Track last time a digest was sent
- [ ] Add caching to prevent ratelimiting on already fetched RSS feeds
- [ ] Set up a scheduled digest publish action
- [ ] Prettify the email
- [ ] Give a way to unsubscribe from a feed from the mail
