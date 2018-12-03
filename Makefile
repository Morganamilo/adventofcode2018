SRC = $(wildcard src/*.rs)
BIN = $(SRC:.rs=)

build: $(BIN)

%: %.rs
	rustc $< -o $@

run-%-1: src/%-1
	./$< < in/$(@:run-%-1=%)

run-%-2: src/%-2
	./$< < in/$(@:run-%-2=%)

clean:
	find src/ -type f  ! -name "*.*"  -delete

