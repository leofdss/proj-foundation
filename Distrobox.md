## Container

```bash
mkdir ~/projects/proj-foundation-box -p
```

```bash
distrobox create -i ubuntu:24.04 \
	--name proj-foundation-box \
	--home ~/projects/proj-foundation-box/
```

```bash
distrobox enter proj-foundation-box
```

## Dependencies

```bash
sudo apt install git
```

### Rust

```bash
# Download and install Rust:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```bash
# Plugin depedendece
sudo apt install build-essential
```

### Flutter

```bash
sudo apt install curl git unzip xz-utils zip libglu1-mesa
```

```bash
sudo apt install clang cmake ninja-build pkg-config libgtk-3-dev build-essential
```

## Starship

https://starship.rs/#quick-install

## Alias

- Bash `~/.bashrc`
- Fish `~/.config/fish/config.fish`
- Zsh `~/.zshrc`

```bash
alias code="codium"
```

```bash
alias podman="distrobox-host-exec podman"
```
