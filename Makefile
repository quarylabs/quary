.PHONY: bump_version_patch
bump_version_patch: ## Bumps the extension version as a patch
	npm version patch --prefix js/packages/quary-extension

.PHONY: proto
proto: proto_fmt ## Generate the proto code
	rm -rf proto/gen
	buf lint
	buf build --exclude-source-info -o -#format=json | jq '.file[] | .package'
	buf generate
	ln proto/.hacking/Cargo.toml proto/gen/rust/Cargo.toml
	mv proto/gen/rust/src/quary.service.v1.rs proto/gen/rust/src/lib.rs
	cargo fmt
	rm -rf js/packages/proto/src/generated/
	mkdir -p js/packages/proto/src/generated/
	cp -a ./proto/gen/ts/. js/packages/proto/src/generated/
	pnpm run fmt

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
	cd rust/core/src/init && sqruff lint .
	cd rust/core/src/init_duckdb && sqruff lint .

.PHONY: rust_build
rust_build: ## Builds the rust code
	cargo build --locked
	
.PHONY: rust_build_wasm
rust_build_wasm: ## Builds the rust wasm code
	cargo build --locked --target=wasm32-unknown-unknown --release --package="quary-wasm-bindgen"
	wasm-bindgen --out-dir=js/packages/quary-extension/src/rust_wasm --target=web --omit-default-module-path "target/wasm32-unknown-unknown/release/quary_wasm_bindgen.wasm"

.PHONY: rust_fmt
rust_fmt: ## Formats the rust code
	cargo fmt

.PHONY: rust_lint
rust_lint: ## Lints the rust code
	cargo fmt --check && cargo clippy

.PHONY: rust_test
rust_test: ## Runs the rust tests
	cargo test

.PHONY: rust_ci
rust_ci: rust_test rust_fmt rust_lint rust_build check_versions_match ## Runs the rust ci commands

.PHONY: markdown_lint
markdown_lint: ## Lints markdown
	docker run -v $(shell pwd):/workdir ghcr.io/igorshubovych/markdownlint-cli:latest "docs/**/*.md"

.PHONY: markdown_lint_fix
markdown_lint_fix: ## Fixes markdown lint errors
	docker run -v $(shell pwd):/workdir ghcr.io/igorshubovych/markdownlint-cli:latest "docs/**/*.md" --fix

.PHONY: ci
ci: ratchet_check proto rust_ci proto_breaking sql_lint_template ## Runs everything
	pnpm install
	pnpm run ci

.PHONY: act
act: ## Runs act which runs the ci locally
	act -P ubuntu-latest=catthehacker/ubuntu:act-latest

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

.PHONY: check_versions_match
check_versions_match: ## Checks the version of the extension matches the CLI
	./.hacking/scripts/check_versions_match.sh $(GITHUB_RELEASE_VERSION)

.PHONY: prettier_fmt
prettier_fmt: ## Formats all the yaml files
	pnpm prettier --write **/*.{yaml,yml}

.PHONY: prettier_lint
prettier_lint: ## Lints all the yaml files
	pnpm prettier --check **/*.{yaml,yml}

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-z.A-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
