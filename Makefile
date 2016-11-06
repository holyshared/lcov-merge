TESTS_DIR = tests
FIXTURES_DIR = $(TESTS_DIR)/fixtures

.PHONY: build

build:
	lcov --config-file=$(TESTS_DIR)/.lcovrc \
	--checksum\
  -a $(FIXTURES_DIR)/fixture1.info\
  -a $(FIXTURES_DIR)/fixture2.info\
  -o $(FIXTURES_DIR)/merged_fixture.info

	lcov --config-file=$(TESTS_DIR)/.lcovrc \
	--checksum\
  -a $(FIXTURES_DIR)/fixture1.info\
  -a $(FIXTURES_DIR)/without_test_name_fixture.info\
  -o $(FIXTURES_DIR)/marged_without_test_name_fixture.info

.PHONY: clean

clean:
	rm $(FIXTURES_DIR)/merged_fixture.info $(FIXTURES_DIR)/marged_without_test_name_fixture.info
