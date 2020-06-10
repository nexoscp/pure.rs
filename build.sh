cargo build --release
strip target/release/pure
ls -al target/release/pure
./target/release/pure
echo $?