all:
	cargo build
	sudo setcap cap_net_admin=eip ./target/debug/rust-practice
	./target/debug/rust-practice &
	ip addr add dev tun1 192.168.0.1/32
	echo "setting tun1 to up"
	ip link set tun1 state up
	pkill rust-practice
