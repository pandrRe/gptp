cargo build --release
rm ./demo/machine/gptp_app
cp ./target/release/main ./demo/machine/gptp_app
cd demo
docker-compose up -d --build