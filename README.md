# Devndat

> Decrypt the `.vndat` file

Only test in [U-ena](https://store.steampowered.com/app/1744570/Uena/).

## Build

```bash
cargo build --release
```

## Usage

```text
Usage: devndat --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>    Input vndat file
  -o, --output <OUTPUT>  Output folder
  -h, --help             Print help
  -V, --version          Print version
```

## Note

I am not sure with file that size smaller than 100 bytes

## Thanks

- https://github.com/morkt/GARbro/issues/440
