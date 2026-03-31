# Usage:
#   make run -- --help
#   make run -- --verbose --port 8080

# Capture everything after 'run' as arguments
RUN_ARGS := $(filter-out run,$(MAKECMDGOALS))

run:
	cd ../../apps/kazi && cargo build && cd ../../examples/demo-project && ../../apps/kazi/target/debug/kazi $(RUN_ARGS)

# Catch-all to prevent "No rule to make target" errors
%:
	@:

.PHONY: run
