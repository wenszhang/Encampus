# Encampus 

Encampus is a tool with the main purpose of streamlining communication within university classes, enabling students to ask both public and private questions to course staff outside of regular course hours. See [Abstract](https://capstone-cs.eng.utah.edu/groups/encampus/-/wikis/Design-Document#abstract) for more details.


## Requirements

If you don't have `cargo-leptos` installed you can install it with

```bash
cargo install cargo-leptos
```

## Running Encampus locally

```bash
cargo leptos watch
```

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing Encampus
```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.  
Tests are located in end2end/tests directory.

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
encampus
site/
```
Set the following environment variables (updating for your project as needed):
```text
LEPTOS_OUTPUT_NAME="encampus"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.
