.PHONY: test fmt vet

test:
	cargo test --workspace
	go test -v ./...

fmt:
	cargo fmt --all -- --check
	gofmt -l .
