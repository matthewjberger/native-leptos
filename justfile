set windows-shell := ["powershell.exe"]

# Run desktop app
[working-directory: 'site']
run:
  npx tailwindcss -i public/input.css -o public/output.css
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

# Setup site
[working-directory: 'site']
setup-site:
  npm install

# Setup gallery
[working-directory: 'component-gallery']
setup-gallery:
  npm install

# Setup all
setup: setup-site setup-gallery

# Format all code
format:
  cargo fmt --all

# Lint all code
lint:
  cargo clippy --all -- -D warnings

# Check all code
check:
  cargo check --all
