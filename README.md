
![Logo](./assets/icon.png)

# CLI Unicron VPN/Proxy

The project introduces a vpn/proxy client, right in the operating system console, for user security.

The CLI only records your connections, encrypted in a hidden file. The full path will be displayed when viewing the list of your connections.

This CLI is designed for free use and complete user privacy.

## Environment Variables

To run this project, you will need to add the following environment variables to your .env file

`ENCRYPTION_KEY` - sizing 32 bites

## Installation

To launch the project, you need to install the following technology stack

- Download [Rust](https://www.rust-lang.org/tools/install)
- Download [Docker](https://www.docker.com/products/docker-desktop/)

The libraries will install themselves when the project starts.
```bash
# Clone repository:
git clone git@github.com:werobin05/CLI-Unicron-VPN-Proxy.git

# For the script to work build_all.sh
cargo install cross --git https://github.com/cross-rs/cross

# Run project on your pc:
cargo run

#Building project:
cargo build
```

## Deployment

To launch the project, you need to install the following technology stack

- Download [Rust](https://www.rust-lang.org/tools/install)
-


```bash
  npm run deploy
```


## Badges

[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge&logo=opensourceinitiative&logoColor=white)](https://choosealicense.com/licenses/mit/)

![Rust](https://img.shields.io/badge/-Rust-gray?logo=rust&logoColor=black&labelColor=orange&style=for-the-badge)


## Feedback

If you have any feedback, please write to [issues](https://github.com/werobin05/CLI-Unicron-VPN-Proxy/issues/new).
## Authors

- github [@werobin05](https://github.com/werobin05)