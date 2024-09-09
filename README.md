# Oliver's Toolbox

This repo contains simple & re-usable rust boilerplate/helpers.

## Features

By default nothing is enabled (to not bloat your dependency tree). Below is a
list of features:

| Feature   | Description                                     |
| --------- | ----------------------------------------------- |
| `tracing` | Tracing setup function with rolling log support |
| `version` | Standardized clap & tracing version messages    |

### Version

To use the version feature, you will need the following `build.rs`:

```rust
use vergen_git2::{CargoBuilder, Emitter, Git2Builder, RustcBuilder};

fn main() {
    let cargo = CargoBuilder::all_cargo().unwrap();
    let git2 = Git2Builder::default().all().sha(true).build().unwrap();
    let rustc = RustcBuilder::all_rustc().unwrap();

    Emitter::default()
        .add_instructions(&cargo)
        .unwrap()
        .add_instructions(&rustc)
        .unwrap()
        .add_instructions(&git2)
        .unwrap()
        .emit()
        .unwrap();
}
```

The following clap derive:

```rust
#[derive(Parser)]
#[command(version = toolbox::version!(), long_version = toolbox::long_version!())]
pub(crate) struct Args {}
```

Produces the following version outputs:

`my-crate -V`:

```txt
my-crate 2.0.0 (b142e42 2024-09-09T12:52:12.000000000Z)
```

`my-crate --version`:

```txt
my-crate
Version:       2.0.0 (b142e42 2024-09-09T12:52:12.000000000Z)
Rustc Version: 1.86.0
Rustc Host:    x86_64-unknown-linux-gnu
Cargo Target:  x86_64-unknown-linux-gnu

feat: support message-pack encoding
```
