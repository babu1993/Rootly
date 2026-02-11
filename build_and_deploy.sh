if command -v cargo >/dev/null 2>&1; then
    VERSION=$(cargo --version)
    echo "✅ Cargo is installed: $VERSION"
else
    echo "❌ Cargo could not be found."
    echo "Hint: If you just installed Rust, try restarting your terminal or running 'source \$HOME/.cargo/env'"
    exit 1
fi