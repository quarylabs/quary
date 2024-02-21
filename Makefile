.PHONY: proto
proto: proto_fmt ## Generate the proto code
	rm -rf proto/gen
	buf lint
	buf build --exclude-source-info -o -#format=json | jq '.file[] | .package'
	buf generate
	ln proto/.hacking/Cargo.toml proto/gen/rust/Cargo.toml
	mv proto/gen/rust/src/quary.service.v1.rs proto/gen/rust/src/lib.rs
	cargo fmt

.PHONY: proto_breaking
proto_breaking: ## Check for breaking changes in the proto code
	buf breaking --against '.git#branch=main'

.PHONY: proto_fmt
proto_fmt: ## Format the proto code
	buf format -w

.PHONY: sql_fix_template
sql_fix_template: ## Fix the sql code
	cd rust/core/src/init && sqlfluff fix .
	cd rust/core/src/init_duckdb && sqlfluff fix .

.PHONY: sql_lint_template
sql_lint_template: ## Lint the sql code
	cd rust/core/src/init && sqlfluff lint .
	cd rust/core/src/init_duckdb && sqlfluff lint .

.PHONY: rust_build
rust_build: ## Builds the rust code
	cargo build

.PHONY: rust_fmt
rust_fmt: ## Formats the rust code
	cargo fmt

.PHONY: rust_lint
rust_lint: ## Lints the rust code
	cargo fmt --check && cargo clippy

.PHONY: rust_test
rust_test: ## Runs the rust tests
	cargo nextest run

.PHONY: rust_ci
rust_ci: rust_test rust_fmt rust_lint rust_build ## Runs the rust ci commands

.PHONY: ci
ci: ratchet_check proto rust_ci proto_breaking sql_lint_template ## Runs everything

.PHONY: ratchet_pin
ratchet_pin: ## Pins all the Github workflow versions
	ratchet pin .github/workflows/*

.PHONY: ratchet_update
ratchet_update: ## Updates all the Github workflow versions
	ratchet update .github/workflows/*

.PHONY: ratchet_check
ratchet_check: ## Checks all the Github workflow versions
	ratchet check .github/workflows/*

.PHONY: bash_lint
bash_lint: ## Lints all the bash scripts
	./.hacking/scripts/bash_lint.sh

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-z.A-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'