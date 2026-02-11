if command -v rustc >/dev/null 2>&1; then
    echo "Rust is installed: $(rustc --version)"
else
    echo "Rust is not installed."
fi