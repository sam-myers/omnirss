# OmniRSS

Converts Spotify show feeds into RSS.


## How to Use

Direct your RSS client to `https://rss.subtlesoftware.dev/spotify/id/$SHOW_ID`.

TODO: Method to search for show ID.

Example: [Joe Rogan](https://rss.subtlesoftware.dev/spotify/id/4rOoJ6Egrf8K2IrywzwOMk)


## Engineering

### Architecture

![architecture](https://user-images.githubusercontent.com/5410234/160204775-f5efb737-ce6a-4698-a603-6bc159f56608.png)

Backend is hosted in [Render](https://render.com/).

Redis is hosted in [Upstash](https://upstash.com/).

Secrets are hosted in [Doppler](https://doppler.com/).


### Deploy to Production

Merges to `master` are automatically deployed.


### Local Development

Test with `cargo test`.

Download the `doppler` CLI to use the development secrets. Start with `doppler run -- cargo run`.

Without the dev secrets, 


### Quality Control

CI ensures that the following pass:
- `cargo test`
- `cargo fmt`
- `cargo clippy`
