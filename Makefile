TESTS_DIR = tests
FIXTURES_DIR = $(TESTS_DIR)/fixtures

.PHONY: build

build:
	lcov --config-file=$(TESTS_DIR)/.lcovrc\
  -a $(FIXTURES_DIR)/fixture1.info\
  -a $(FIXTURES_DIR)/fixture2.info\
  -o $(FIXTURES_DIR)/merged_fixture.info

.PHONY: clean

clean:
	rm tests/fixtures/merged_fixture.info
