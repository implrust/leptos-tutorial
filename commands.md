# environment setup commands
- cargo install trunk
- rustup target add wasm32-unknown-unknown
- cargo install leptosfmt

# vscode plugins
- leptos-fmt
- leptos-vscode

# project setup
- cargo init leptos-tutorial
- cargo add leptos --features=csr
- cargo add console_error_panic_hook

# project run commands
- trunk serve open
- trunk serve --port 3000 --open

# leptos csr starter template
- https://github.com/leptos-rs/start-trunk

# cargo commands
- cargo --list

