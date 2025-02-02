dev:
  cargo run

start:
  cargo run --release

convex:
  pnpm dlx convex dev --tail-logs

watch:
  cargo watch -x 'run'

setup:
    cargo binstall cargo-watch
    pnpm dlx convex dev --tail-logs