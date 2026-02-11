if command -v rustup >/dev/null 2>&1; then
    echo "Rustup is installed: $(rustup --version)"
else
    echo "Rustup is not installed."
fi