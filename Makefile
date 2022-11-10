.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

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
