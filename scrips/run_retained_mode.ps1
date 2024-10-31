# Set path to the Cargo.toml in the project root
$manifestPath = Join-Path (Split-Path -Path $MyInvocation.MyCommand.Path -Parent) "..\Cargo.toml"

# Run cargo with retained-mode feature
cargo run --manifest-path $manifestPath --no-default-features --features retained-mode
