# OmniRSS

Converts Spotify show feeds into RSS.


## Engineering

Backend is hosted in [Netlify](https://netlify.com/).

Secrets are hosted in [Doppler](https://doppler.com/).


### Deploy to Production

Merges to `master` are automatically deployed.


### Local Development

Prerequisites:
- Download the `netlify` CLI to build & run the serverless functions locally.
- Download the `doppler` CLI for development secrets.

Start with `doppler run -- netlify dev`.
