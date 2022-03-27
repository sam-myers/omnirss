# OmniRSS

Converts Spotify show feeds into RSS.


## Engineering

### Architecture

![architecture](https://user-images.githubusercontent.com/5410234/160204775-f5efb737-ce6a-4698-a603-6bc159f56608.png)

Backend is hosted in [Render](https://render.com/).

Redis is hosted in [Upstash](https://upstash.com/).

Secrets are hosted in [Doppler](https://doppler.com/).

End-to-end test results on [Cypress Dashboard](https://dashboard.cypress.io/projects/srh5kf/runs).


### Deploy to Production

Merges to `master` are automatically deployed.


### Local Development

Test with `cargo test`.

Download the `doppler` CLI to use the development secrets. Start with `doppler run -- cargo run`.


### Quality Control

All of the following run in CI on pull request.

| Test              | How to Run Locally          | Details                                    |
|-------------------|-----------------------------|--------------------------------------------|
| Unit Tests        | `cargo test`                | Rust, self-contained                       |
| Integration Tests | `cargo test`                | Rust, uses mocked services                 |
| Cypress           | `yarn run cypress open`     | Cypress, browser automation, real services |
| Lints             | `cargo fmt`, `cargo clippy` | Applies to Rust components                 |
