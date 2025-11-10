cargo build --release
sudo cp target/release/cfs /usr/local/bin/
echo "CFS is now installed to /usr/local/bin/cfs"
echo "Read the README.md for obvious reasons"
echo ""
echo "There is a possibility that it didn't actually install"
echo "If it didn't, make sure you have Rust's cargo installed"
echo "Or if it did compile, but didn't install, make sure you have sudo installed and have permission to use it"
