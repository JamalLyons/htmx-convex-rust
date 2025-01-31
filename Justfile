run:
  cargo run

convex:
  pnpm dlx convex dev --tail-logs

watch:
  cargo watch -x 'run'

setup-project:
    exit

setup-deps:
    cargo binstall cargo-watch
