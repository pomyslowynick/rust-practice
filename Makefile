all:
	cargo build
	sudo setcap cap_net_admin=eip ./target/debug/rust-practice
	./target/debug/rust-practice &
	sudo ip addr add dev tun1 192.168.0.2/26
	echo "setting tun1 to up"
	sudo ip link set dev tun1 up
	sleep 5
	pkill rust-practice

build:
	cargo build
