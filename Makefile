SHELL := /bin/bash
EXE := target/release/sudoku-rs

ifdef CI
	HYPERFINE_COLOR := --style color
endif
N := 50
TEST_FILE := tests/test$(N)
ANS_FILE  := tests/answer$(N)

$(EXE):
	cargo build --release

.PHONY: clean
clean:
	cargo clean

.PHONY: run
run:
	cargo run --release

.PHONY: verify
verify: $(EXE)
	@echo "Testing $(TEST_FILE)..."
	@diff $(ANS_FILE) <(echo "$(TEST_FILE)" | $(EXE)) && echo -e "\033[1;32mYeah!!! Everything looks alright!\033[0m"

.PHONY: bench
bench: $(EXE)
	@hyperfine "make verify N=50" "make verify N=1000" "make verify N=10000" --export-json benchmark_report.json $(HYPERFINE_COLOR)

.PHONY: test
test: $(EXE)
	./Lab1.sh test_group answer_group

all: $(EXE)
