# roadiebrodie
your interactive travel compendium

Steven Zhang
Michael Lin
Brandon Choy
Miccah Castorina

Omg I rebased! -Steven

I REBASED TOO -MICHAEL

🗑️ - Brandon


## building

1. compile tailwind

```bash
npx tailwindcss --input tailwind/styles.css --output static/styles.css --config tailwind/config.js
```

2. compile roadiebrodie

```bash
cargo build
```


## running

you can run either directly run the compiled binary or use `cargo` to run it.

```bash
# directly running binary
./target/debug/roadiebrodie

# using cargo
cargo run
```
