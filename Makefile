run:
	go run cmd/main.go

watch:
	nodemon --watch './**/' --ext 'go,txt' --signal SIGTERM --exec '/usr/bin/clear && /usr/local/go/bin/go' run ./cmd/main.go

