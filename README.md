# rust-devcontainer

Rust devcontainer definition for [VS Code Remote - Containers][remote-containers] extension to easily develop locally in the same environment used by the CodeRunner.

If you're not familiar with devcontainers, see [Developing inside a Container](https://code.visualstudio.com/docs/remote/containers).

## Usage

Open this project in a devcontainer with [VS Code Remote - Containers][remote-containers] extension.

Put your solution and tests in `src/lib.rs`:
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// -------------------- 8< --------------------
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        assert_eq!(add(1, 1), 2);
    }
}
```

Then run `cargo test`. If the kata uses preloaded code, create `lib/preloaded.rs`.

## Extensions

Only [`rust-analyzer`][rust-analyzer] and [`Better TOML`][better-toml] are installed.

To install more extensions in the devcontainer, configure default extensions in _your_ VS Code settings:

```jsonc
    "remote.containers.defaultExtensions": [
        "eamodio.gitlens",
    ],
```

[remote-containers]: https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers
[rust-analyzer]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
[better-toml]: https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml
