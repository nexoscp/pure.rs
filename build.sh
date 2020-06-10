cargo build --release
strip target/release/pure
readelf -h ./target/release/pure
ls -alh target/release/pure
./target/release/pure
echo $?