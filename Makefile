GREEN := \033[0;32m
RESET := \033[0m

run:
	@cargo run --release
	
rustup:
	@command -v rustup >/dev/null 2>&1 || curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal
	@echo "$(GREEN)Run: source $$HOME/.cargo/env$(RESET)\n"