set windows-shell := ["powershell.exe"]

# Run desktop app
[working-directory: 'site']
run:
  trunk build --release
  cargo run --manifest-path ../Cargo.toml

# Run component gallery
[working-directory: 'component-gallery']
gallery:
  trunk serve

# Run API
[working-directory: 'api']
api:
  cargo run
