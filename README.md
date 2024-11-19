# my-site

A fast, lightweight, and easy to deploy web application built with 
[axum](https://github.com/tokio-rs/axum),
[sqlite](https://github.com/sqlite/sqlite),
[htmx](https://github.com/bigskysoftware/htmx),
[tailwindcss](https://github.com/tailwindlabs/tailwindcss).

## Installation

Run

```shell
curl https://github.com/yoonthegoon/my-site/install.sh | bash
```

and select the prompted environment you require.

### dev

You'll have to ensure you've got [tmux](https://github.com/tmux/tmux/wiki) installed.
Instructions can be found [here](https://github.com/tmux/tmux/wiki/Installing).

### release

You'll have to ensure you've got [nginx](https://github.com/nginx/nginx) installed.
Instructions can be found [here](https://github.com/nginx/nginx?tab=readme-ov-file#downloading-and-installing).

## Usage

### dev

Run

```shell
npm run dev
```

to run live server watchers for `tailwindcss` and `my-site`.
Any changes to certain files will automatically restart either application, letting you rapidly iterate through changes.

### release

[//]: # (TODO: config nginx)

After all the configuration changes have been made, you can run any of the following:

```shell
npm run reload
npm run restart
npm run start
npm run status
npm run stop
```
