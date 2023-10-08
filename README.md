# Super Tic-Tac-Toe

A more complex version of tic-tac-toe with nested game boards. 

See more about it in this [Vsauce video](https://www.youtube.com/watch?v=_Na3a1ZrX7c).

## Usage

### Building Tailwind CSS File

```bash
./tailwindcss --watch --output ./app.css
```

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

## License

The template ships with both the Apache and MIT license.
If you don't want to have your app dual licensed, just remove one (or both) of the files and update the `license` field in `Cargo.toml`.

There are two empty spaces in the MIT license you need to fill out: `` and `Raven <sd5356@rit.edu>`.

[trunk]: https://github.com/thedodd/trunk
