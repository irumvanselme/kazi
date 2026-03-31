# Usage:
#   make run -- --help
#   make run -- --verbose --port 8080

run:
	cd ../../apps/kazi && cargo build && cd ../../examples/demo-project && ../../apps/kazi/target/debug/kazi $(ARGS)

# Catch-all to prevent "No rule to make target" errors
%:
	@:

.PHONY: run
