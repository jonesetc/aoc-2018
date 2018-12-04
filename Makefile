PAIRS := $(foreach\
	x,\
	$(subst day,,$(wildcard day*)),\
	$(x)-1 $(x)-2\
)

LINTS := $(addprefix lint-,$(PAIRS))
TESTS := $(addprefix test-,$(PAIRS))
BUILDS := $(addprefix build-,$(PAIRS))
RUNS := $(addprefix run-,$(PAIRS))

day = $(word 2,$(subst -, ,$1))
part = $(word 3,$(subst -, ,$1))

.PHONY: $(LINTS) lint-all
$(LINTS):
	cargo clippy \
		--package "day$(call day,$@)" \
		--bin "part$(call part,$@)" \
		-- -D clippy::pedantic
lint-all: $(LINTS)

.PHONY: $(TESTS) test-all
$(TESTS):
	cargo test \
		--package "day$(call day,$@)" \
		--bin "part$(call part,$@)"
test-all: $(TESTS)

.PHONY: $(BUILDS) build-all
$(BUILDS):
	cargo build \
		--package "day$(call day,$@)" \
		--bin "part$(call part,$@)" \
		--release
build-all: $(BUILDS)

.PHONY: $(RUNS) run-all
$(RUNS):
	cargo run \
		--package "day$(call day,$@)" \
		--bin "part$(call part,$@)"
run-all: $(RUNS)
