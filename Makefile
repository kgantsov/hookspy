start_backend:
	cd backend && cargo run

start_frontend:
	cd frontend && trunk serve

build_backend:
	cd backend && cargo build --release

build_frontend:
	cd frontend && trunk build --release
