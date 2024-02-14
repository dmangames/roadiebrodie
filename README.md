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

1. compile roadiebrodie

`cargo` uses a `build` script which first compiles tailwind. this uses `npx`
which must be installed on your system.

```bash
cargo build
```

## running

you can either directly run the compiled binary or use `cargo` to run it.

```bash
# directly running binary
./target/debug/roadiebrodie

# using cargo
cargo run
```

## hot-reloading

use the following command to have `cargo` rebuild and rerun the project when
any files change.

```bash
cargo watch -x run
```
