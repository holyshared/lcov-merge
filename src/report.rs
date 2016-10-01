use std::io:: { Error as IOError };
use std::convert::From;
use std::result:: { Result };
use std::collections:: { HashMap };
use std::default::Default;
use lcov_parser::parser:: { LCOVParser, RecordParseError };
use lcov_parser::record:: { LCOVRecord };
use branch:: { Branch, BranchUnit };

pub enum ParseError {
    IOError(IOError),
    RecordParseError(RecordParseError)
}

impl From<IOError> for ParseError {
    fn from(error: IOError) -> Self {
        ParseError::IOError(error)
    }
}

impl From<RecordParseError> for ParseError {
    fn from(error: RecordParseError) -> Self {
        ParseError::RecordParseError(error)
    }
}

pub fn records_from_file(file: &str) -> Result<Vec<LCOVRecord>, ParseError> {
    let parser = try!(LCOVParser::from_file(file));
    Ok(try!(parser.parse()))
}

//
// $testcount = $testdata->{$testname};
// $testfnccount = $testfncdata->{$testname};
// $testbrcount = $testbrdata->{$testname};
//

#[derive(Clone)]
struct Test {
    test_count: HashMap<u32, u32>, // key: line number, value: test count
    test_fn_count: HashMap<String, u32>, // key: function name, value: line number
    test_br_count: u32 // FIXME br data structure
}

impl Default for Test {
    fn default() -> Self {
        Test {
            test_count: HashMap::new(),
            test_fn_count: HashMap::new(),
            test_br_count: 0
        }
    }
}

// my $sumcount
// my $sumbrcount
// my $sumfnccount


struct SumCount {
    sum_count: HashMap<u32, u32>, // key: line number, value: count
    sum_br_count: HashMap<String, Branch>, // key: function name, value: Branch
    sum_fn_count: HashMap<String, u32> // FIXME br data structure
}

// key: line_number, value: checksum value
type CheckSum = HashMap<u32, String>;

// key: function name, value: line_number
type FunctionData = HashMap<String, u32>;

struct ReportParser {
    test_name: Option<String>,
    source_name: Option<String>,
    test: Option<Test>,
    tests: HashMap<String, Test>,
    sum: SumCount,
    checksum: CheckSum,
    func: FunctionData
}

impl ReportParser {
    fn parse(&mut self, file: &str) -> Result<(), ParseError> {
        let records = try!(records_from_file(file));

        for record in records.iter() {
            match record {
                &LCOVRecord::TestName(ref name) => self.on_test_name(name),
                &LCOVRecord::SourceFile(ref name) => self.on_source_file(name),
                &LCOVRecord::Data(ref line_number, ref execution_count, ref checksum) => self.on_data(
                    line_number,
                    execution_count,
                    checksum,
                ),
                &LCOVRecord::FunctionName(ref line_number, ref func_name, ) => self.on_func_name(
                    func_name,
                    line_number
                ),
                &LCOVRecord::FunctionData(ref execution_count, ref func_name) => self.on_func_data(
                    func_name,
                    execution_count
                ),
                _ => { continue; }
            };
        }
        Ok(())
    }
    fn on_test_name(&mut self, test_name: &Option<String>) {
        self.test_name = test_name.clone();
    }
    fn on_source_file(&mut self, source_name: &String) {
        self.source_name = Some(source_name.clone());

        match self.test_name {
            Some(ref current_test_name) => {
                // $testcount = $testdata->{$testname};
                // $testfnccount = $testfncdata->{$testname};
                // $testbrcount = $testbrdata->{$testname};
                if !self.tests.contains_key(current_test_name) {
                    self.tests.insert(current_test_name.to_string(), Test::default());
                }
                let test = self.tests.get(current_test_name);
                self.test = Some(test.unwrap().clone());
            },
            None => self.test = Some(Test::default())
        }
    }
    fn on_data(&mut self, line_number: &u32, execution_count: &u32, checksum: &Option<String>) {
        let mut sum_count = self.sum.sum_count.entry(line_number.clone()).or_insert(0);
        *sum_count += *execution_count;

        if self.test_name.is_some() {
            if self.test.is_some() {
                let mut test = self.test.as_mut().unwrap();
                let mut test_count = test.test_count.entry(line_number.clone()).or_insert(0);
                *test_count += *execution_count;
            }
        }

        if checksum.is_none() {
            return;
        }

        if !self.checksum.contains_key(line_number) {
            let checksum_value = checksum.clone().unwrap();
            self.checksum.insert(line_number.clone(), checksum_value);
            return;
        }

        let checksum_value = checksum.clone().unwrap();
        let current_checksum = self.checksum.get(line_number).unwrap();
        if current_checksum != &checksum_value {
            println!("{} {}", current_checksum, checksum_value);
        }
    }
    fn on_func_name(&mut self, func_name: &String, line_number: &u32) {
        let _ = self.func.entry(func_name.clone())
            .or_insert(line_number.clone());

        let _ = self.sum.sum_fn_count.entry(func_name.clone())
            .or_insert(0);

        if self.test_name.is_some() {
            if self.test.is_some() {
                let mut test = self.test.as_mut().unwrap();
                let _ = test.test_fn_count.entry(func_name.clone()).or_insert(0);
            }
        }

        if !(self.test_name.is_some() && self.test.is_some()) {
            return;
        }
        let mut test = self.test.as_mut().unwrap();
        let _ = test.test_fn_count.entry(func_name.clone()).or_insert(0);
    }
    fn on_func_data(&mut self, func_name: &String, execution_count: &u32) {
        let mut sum_fn_count = self.sum.sum_fn_count.entry(func_name.clone()).or_insert(0);
        *sum_fn_count += *execution_count;

        if self.test_name.is_none() {
            return;
        }

        let mut test = self.test.as_mut().unwrap();
        let mut test_fn_count = test.test_fn_count.entry(func_name.clone()).or_insert(0);
        *test_fn_count += *execution_count;
    }
    fn on_branch_data(line_number: &u32, block_number: &u32, branch_number: &u32, taken: &u32) {
/*
        if self.sum.sum_br_count.contains_key(line_number) {
            let mut branch = self.sum.sum_br_count.get_mut(line_number).unwrap();

            if (branch.block == block_number && branch.branch == branch_number) {
            }
        } else {
            self.sum.sum_br_count.insert(line_number.clone(), Branch {
                block: block_number.clone(),
                branch: branch_number.cline(),
                taken: taken.cline()
            }
        }
*/
/*
        let mut branch = self.sum.sum_br_count.entry(line_number.clone())
            .or_insert(Branch {
                block: block_number.clone(),
                branch: branch_number.cline(),
                taken: taken.cline()
            });
*/


//BranchData(u32, u32, u32, u32),   // BRDA:<line number>,<block number>,<branch number>,<taken>

/*
        my ($line, $block, $branch, $taken) =
           ($1, $2, $3, $4);

        last if (!$br_coverage);
        $sumbrcount->{$line} =
            br_ivec_push($sumbrcount->{$line},
                     $block, $branch, $taken);

        # Add test-specific counts
        if (defined($testname)) {
            $testbrcount->{$line} =
                br_ivec_push(
                    $testbrcount->{$line},
                    $block, $branch,
                    $taken);
        }
        last;
*/



    }
}

















/*
sub br_ivec_push($$$$)
{
    my ($vec, $block, $branch, $taken) = @_;
	my $offset;
	my $num = br_ivec_len($vec);
	my $i;

	$vec = "" if (!defined($vec));
	$block = $BR_VEC_MAX if $block < 0;

	# Check if branch already exists in vector
	for ($i = 0; $i < $num; $i++) {
		my ($v_block, $v_branch, $v_taken) = br_ivec_get($vec, $i);
		$v_block = $BR_VEC_MAX if $v_block < 0;

		next if ($v_block != $block || $v_branch != $branch);

		# Add taken counts
		$taken = br_taken_add($taken, $v_taken);
		last;
	}

	$offset = $i * $BR_VEC_ENTRIES;
	$taken = br_taken_to_num($taken);

	# Add to vector
	vec($vec, $offset + $BR_BLOCK, $BR_VEC_WIDTH) = $block;
	vec($vec, $offset + $BR_BRANCH, $BR_VEC_WIDTH) = $branch;
	vec($vec, $offset + $BR_TAKEN, $BR_VEC_WIDTH) = $taken;

	return $vec;
}
*/
