# BEU is a plateform for being both yourself and professional self 

# develop

```sh
rustup add rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
cargo leptos watch
```

# deploy

```sh
# build image
podman build --target runner -t beu .
# run container
podman run -itd -p 3000:3000 --name beu beu
```