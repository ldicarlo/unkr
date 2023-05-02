.PHONY: help

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

run: test ## run the app backend
	@cargo run

run-brute-force: test ## run the app backend
	@cargo run -- brute-force --string=OBKRUOXOGHULBSOLIFBBWFLRVQQPRNGKSSOTWTQSJQSSEKZZWATJKLUDIAWINFBNYPVTTMZFPKWGDKZXTJCDIGKUHUAUEKCAR

get-decryptors: test ## print current decryptors
	@cargo run -- get-decryptors

test: ## test the app backend
	@cargo test

watch: ## Use ENTR to reload and run tests
	@find src -type f | entr -cr make run

run-brute-force-test: test ## run test
	@cargo run -- brute-force --string=TQDJMH

solve-k1: ## Solve K1
	@cargo run -- decrypt --string "$$(cat panels/kr1)" -- vigenere:PALIMPSEST:KRYPTOS

solve-k2: ## Solve K2
	@cargo run -- decrypt --string "$$(cat panels/kr2)" -- vigenere:ABSCISSA:KRYPTOS

solve-k3: ## Solve K3
	@cargo run -- decrypt --string "$$(cat panels/kr3clear)" -- transpose:24 reverse join transpose:8 reverse join

brute-force-k3: ## Bruteforce k3
	@cargo run -- brute-force --string "$$(cat panels/kr3clear)"
