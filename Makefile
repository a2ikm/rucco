CFLAGS=-std=c11 -g -static

rucco:
	cargo build

.PHONY: test
test: rucco
	./test.sh

.PHONY: clean
clean:
	rm -f tmp*
