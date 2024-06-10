dev:
	cargo tauri dev

build:
	cargo tauri build --features custom-protocol

update:
	cargo update --manifest-path=src-tauri/Cargo.toml

compile:
	docker compose  -f "docker-compose.yaml" up --build rust_cross_compile_msvc 
