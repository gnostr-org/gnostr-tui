cargo-sort:## 	cargo-sort
	@[ -x cargo-sort ] || cargo install cargo-sort
	cargo-sort
cargo-deny-check-bans:## 	cargo-deny-check-bans
	@[ -x cargo-deny ] || cargo install cargo-deny
	cargo deny check bans
