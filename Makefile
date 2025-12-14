.PHONY: build_frontend build_backend build

build_frontend:
	cd frontend && trunk build --release

build_backend: build_frontend
	cd backend && cargo build --release

build: build_backend

start_backend:
	cd backend && cargo run

start_frontend:
	cd frontend && trunk serve
