default:
	cargo build
	just frontend/default

install:
	just frontend/install
	just jhttpd/install
