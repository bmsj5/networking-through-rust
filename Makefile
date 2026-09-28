# Networking Through Rust - Educational Demo Runner
.PHONY: all run-all foundations protocols production clean help

all: help

run-all: foundations protocols production

foundations:
	@echo "Running Foundations Demos..."
	cd code && cargo run --bin tcp-echo-server -- --once
	cd code && cargo run --bin tcp-header-parser
	cd code && cargo run --bin cidr-calculator

protocols:
	@echo "Running Protocol Demos..."
	cd code && cargo run --bin tcp-state-machine
	cd code && cargo run --bin ip-header-parser
	cd code && cargo run --bin dns-resolver

production:
	@echo "Production networking concepts are mostly in chapters/"
	@echo "  (eBPF/XDP, overlays, Anycast — see chapters/)"

release-%:
	cd code && cargo run --release --bin $*

clean:
	cd code && cargo clean

help:
	@echo "Networking Through Rust"
	@echo ""
	@echo "Targets:"
	@echo "  run-all       - Run all demos in learning order"
	@echo "  foundations   - Echo server, TCP header, CIDR"
	@echo "  protocols     - TCP state machine, IP header, DNS"
	@echo "  release-<bin> - Run a demo optimized"
	@echo "  clean         - Clean build artifacts"
	@echo ""
	@echo "Examples:"
	@echo "  make foundations"
	@echo "  make release-tcp-header-parser"
