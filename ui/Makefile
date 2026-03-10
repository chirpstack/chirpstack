.PHONY: build server dependencies clean

build: dependencies
	pnpm build

server: dependencies
	pnpm start

dependencies:
	pnpm install

clean:
	rm -rf build

format:
	pnpm format
