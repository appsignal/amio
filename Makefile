build:
	docker build -t amio-test .

cargo_test:
	docker run --rm -it -v $(shell pwd):/amio amio-test cargo test

cargo_check:
	docker run --rm -it -v $(shell pwd):/amio amio-test cargo check

bash:
	docker run --rm -it -v $(shell pwd):/amio amio-test /bin/bash
