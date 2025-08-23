# required cli tools: taplo-cli,cargo-edit
# Installation by cargo
# cargo install taplo-cli --locked
# cargo install cargo-edit -f --no-default-features --features "set-version"

alias pr := prepare-release
alias pt := push-tag

default:
	just --list --unsorted

toolchain:
	rustup -V
	rustc -V
	cargo -V
	cargo fmt --version
	cargo clippy -V

fmt:
	cargo fmt
	taplo fmt

check:
	cargo fmt --check
	taplo fmt --check
	cargo clippy --no-deps --all-features -- -D warnings

prepare-release tag:
	cargo set-version {{tag}}
	just fmt
	git commit -am "prepare release {{tag}}"

push-tag tag:
	git tag {{tag}}
	git push origin {{tag}}

run *args:
	cargo run -F _dev -- {{args}}
