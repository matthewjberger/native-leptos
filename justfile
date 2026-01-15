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

# Format code
format:
  cargo fmt
  cd site; cargo fmt
  cd ui; cargo fmt
  cd component-gallery; cargo fmt
  cd api; cargo fmt
  cd api-types; cargo fmt
  cd protocol; cargo fmt

# Lint code
lint:
  cargo clippy -- -D warnings
  cd site; cargo clippy -- -D warnings
  cd ui; cargo clippy -- -D warnings
  cd component-gallery; cargo clippy -- -D warnings
  cd api; cargo clippy -- -D warnings
  cd api-types; cargo clippy -- -D warnings
  cd protocol; cargo clippy -- -D warnings
