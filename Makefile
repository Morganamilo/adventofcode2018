SRC = $(wildcard src/*.rs)
BIN = $(SRC:.rs=)
OPTLEVEL=0

build: $(BIN)

%: %.rs
	rustc $< -g -C opt-level=$(OPTLEVEL) -o $@

run-%-1: src/%-1
	./$< < in/$(@:run-%-1=%)

run-%-2: src/%-2
	./$< < in/$(@:run-%-2=%)

run-stdin-%: src/%
	./$<

clean:
	find src/ -type f  ! -name "*.*"  -delete

