# my-site

A fast, lightweight, and easy to deploy web application built with 
[axum](https://github.com/tokio-rs/axum),
[sqlite](https://github.com/sqlite/sqlite),
[htmx](https://github.com/bigskysoftware/htmx),
[tailwindcss](https://github.com/tailwindlabs/tailwindcss).

## Dependencies

- cargo 1.82.0
- sqlite 3.47.0

### dev

- node 18.20.3
- npm 10.7.0

## Installation

```shell
git clone https://github.com/yoonthegoon/my-site.git
cd my-site
```

### dev

```shell
npm install
cargo install --path .
```

### release

```shell
cargo build -r
cp target/release/my-site /usr/local/bin
```

## Usage

### dev

Assuming [tmux](https://github.com/tmux/tmux/wiki) is installed:

```shell
tmux new-session -d -s dev "npx tailwindcss -o ./public/css/style.css --watch"
tmux split-window -h "cargo watch -x run"
tmux attach -t dev
```

If not, run each line in a different terminal sessions:

```shell
npx tailwindcss -o ./public/css/style.css --watch # session 1
cargo watch -x run # session 2
```

[//]: # (### release)
