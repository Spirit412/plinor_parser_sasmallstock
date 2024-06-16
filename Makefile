dev:
	cargo tauri dev

build:
	cargo tauri build --features custom-protocol

update:
	cargo update --manifest-path=src-tauri/Cargo.toml