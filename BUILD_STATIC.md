# Building a static version of lolcow with Rust

To build a statically linked version of lolcow:

## x86_64 (AMD64)

```bash
# Install musl-tools first (requires sudo)
sudo apt-get install musl-tools

# Add musl target for x86_64
rustup target add x86_64-unknown-linux-musl

# Create cargo config to use musl-gcc linker
cat > .cargo/config.toml << 'EOF'
[target.x86_64-unknown-linux-musl]
linker = "musl-gcc"
EOF

# Build for x86_64
cargo build --target x86_64-unknown-linux-musl --release --bin lolcow
```

The static binary will be at:
`./target/x86_64-unknown-linux-musl/release/lolcow`

## ARM64 (aarch64)

ARM64 systems do not require musl-tools or config files. The native linker works with musl.

```bash
# Add musl target for ARM64
rustup target add aarch64-unknown-linux-musl

# Build for ARM64
cargo build --target aarch64-unknown-linux-musl --release --bin lolcow
```

No config file is needed for ARM64 - the native linker works with musl.

The static binary will be at:
`./target/aarch64-unknown-linux-musl/release/lolcow`

## Verification

To verify the binary is statically linked:

```bash
file ./target/.../release/lolcow
# Should show: " statically linked"

ldd ./target/.../release/lolcow
# Should show: "not a dynamic executable"
```

## Size comparison

Static binaries are larger but have no external dependencies:

```
Dynamic (glibc):   ~550 KB
Static (musl):     ~640 KB
```

The ~100KB difference is the musl libc library.
