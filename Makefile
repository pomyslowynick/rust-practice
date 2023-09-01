all:
	cargo build
	sudo setcap cap_net_admin=eip ./target/debug/rust-practice
	./target/debug/rust-practice &
	sudo ip addr add dev tun1 192.168.0.1/32
	echo "setting tun1 to up"
	sudo ip link set dev tun1 up
	sleep 10
	pkill rust-practice
