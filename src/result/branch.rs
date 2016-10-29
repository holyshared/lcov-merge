use std::hash:: { Hash, Hasher };
use std::cmp:: { Eq, PartialEq };
use std::fmt:: { Display, Formatter, Result };
use std::ops::AddAssign;
use std::collections::hash_map:: { Iter };
use std::convert::AsRef;
use lcov_parser:: { BranchData };
use result::summary:: { LineNumber, AggregateResult, Summary, ExecutionCount };
use result::summary::counter:: { HitFoundCounter, FoundCounter, HitCounter };

/// Units of the branch
///
/// # Examples
///
/// ```
/// use lcov_merge::BranchUnit;
///
/// let branch1 = BranchUnit::new(1, 1);
/// let branch2 = BranchUnit::new(1, 1);
///
/// assert!(branch1 == branch2);
///
/// let not_eq_branch1 = BranchUnit::new(1, 1);
/// let not_eq_branch2 = BranchUnit::new(1, 2);
///
/// assert!(not_eq_branch1 != not_eq_branch2);
/// ```
#[derive(Debug, Eq, Clone)]
pub struct BranchUnit(u32, u32);

impl BranchUnit {
    pub fn new(block: u32, branch: u32) -> BranchUnit {
        BranchUnit(block, branch)
    }
}

impl PartialEq for BranchUnit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Hash for BranchUnit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl Display for BranchUnit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BranchBlocks {
    blocks: AggregateResult<BranchUnit, ExecutionCount>
}

impl BranchBlocks {
    pub fn new() -> Self {
        BranchBlocks {
            blocks: AggregateResult::new()
        }
    }
}

impl AsRef<AggregateResult<BranchUnit, ExecutionCount>> for BranchBlocks {
    fn as_ref(&self) -> &AggregateResult<BranchUnit, ExecutionCount> {
        &self.blocks
    }
}

impl Summary<BranchUnit, ExecutionCount> for BranchBlocks {
    fn iter(&self) -> Iter<BranchUnit, ExecutionCount> {
        self.blocks.iter()
    }
    fn contains_key(&self, key: &BranchUnit) -> bool {
        self.blocks.contains_key(key)
    }
    fn get(&self, key: &BranchUnit) -> Option<&ExecutionCount> {
        self.blocks.get(key)
    }
}

impl HitCounter for BranchBlocks {
    fn hit_count(&self) -> usize {
        self.iter()
            .filter(|&(_, taken)| *taken > 0)
            .count()
    }
}

impl FoundCounter for BranchBlocks {
    fn found_count(&self) -> usize {
        self.blocks.len()
    }
}

impl HitFoundCounter for BranchBlocks {}

impl AddAssign for BranchBlocks {
    fn add_assign(&mut self, other: BranchBlocks) {
        self.blocks += other.as_ref();
    }
}

impl<'a> AddAssign<&'a BranchData> for BranchBlocks {
    fn add_assign(&mut self, data: &'a BranchData) {
        let unit = BranchUnit::new(data.block, data.branch);
        let mut block_count = self.blocks.entry(unit)
            .or_insert(0);
        *block_count += data.taken;
    }
}

impl<'a> AddAssign<&'a BranchBlocks> for BranchBlocks {
    fn add_assign(&mut self, other: &'a BranchBlocks) {
        self.blocks += other.as_ref();
    }
}


#[derive(Debug, Clone)]
pub struct Branches {
    branches: AggregateResult<LineNumber, BranchBlocks>
}

impl Branches {
    pub fn new() -> Self {
        Branches {
            branches: AggregateResult::new()
        }
    }
}

impl AsRef<AggregateResult<LineNumber, BranchBlocks>> for Branches {
    fn as_ref(&self) -> &AggregateResult<LineNumber, BranchBlocks> {
        &self.branches
    }
}

impl HitCounter for Branches {
    fn hit_count(&self) -> usize {
        self.iter()
            .map(|(_, blocks)| blocks.hit_count() )
            .fold(0, |p, n| p + n)
    }
}

impl FoundCounter for Branches {
    fn found_count(&self) -> usize {
        self.iter()
            .map(|(_, blocks)| blocks.found_count() )
            .fold(0, |p, n| p + n)
    }
}

impl HitFoundCounter for Branches {}


impl Summary<LineNumber, BranchBlocks> for Branches {
    fn iter(&self) -> Iter<LineNumber, BranchBlocks> {
        self.branches.iter()
    }
    fn contains_key(&self, key: &LineNumber) -> bool {
        self.branches.contains_key(key)
    }
    fn get(&self, key: &LineNumber) -> Option<&BranchBlocks> {
        self.branches.get(key)
    }
}

impl<'a> AddAssign<&'a Branches> for Branches {
    fn add_assign(&mut self, other: &'a Branches) {
        self.branches += other.as_ref();
    }
}

impl<'a> AddAssign<&'a BranchData> for Branches {
    fn add_assign(&mut self, other: &'a BranchData) {
        let mut blocks = self.branches.entry(other.line)
            .or_insert(BranchBlocks::new());
        *blocks += other;
    }
}

#[cfg(test)]
mod tests {
    use std::collections:: { HashMap };
    use lcov_parser:: { BranchData };
    use result::branch:: { BranchUnit, Branches, BranchBlocks };
    use result::summary:: { Summary };
    use result::summary::counter:: { FoundCounter, HitCounter };

    #[test]
    fn branch_unit() {
        let branch1 = BranchUnit(1, 1);
        let branch2 = BranchUnit(1, 2);

        assert!(branch1 != branch2);

        let same_branch1 = BranchUnit(1, 1);
        let same_branch2 = BranchUnit(1, 1);
    
        assert_eq!(same_branch1, same_branch2);
    }

    #[test]
    fn branch_unit_as_hash_key() {
        let mut container = HashMap::new();
        container.insert(BranchUnit(1, 1), 1);

        assert!( container.contains_key(&BranchUnit(1, 1)) );
    }

    #[test]
    fn add_branch_data() {
        let mut branches = BranchBlocks::new();
        let b1 = &BranchData { line: 1, block: 0, branch: 1, taken: 1 };
        let b2 = &BranchData { line: 1, block: 0, branch: 1, taken: 1 };

        branches += b1;
        branches += b2;

        assert_eq!(branches.get(&BranchUnit::new(0, 1)), Some(&2));
    }

    #[test]
    fn append_branches() {
        let mut branches = BranchBlocks::new();
        let b1 = &BranchData { line: 1, block: 0, branch: 1, taken: 1 };
        let b2 = &BranchData { line: 1, block: 0, branch: 1, taken: 1 };

        branches += b1;
        branches += b2;

        let cloned_branches = branches.clone();

        branches += &cloned_branches;

        assert_eq!(branches.get(&BranchUnit::new(0, 1)), Some(&4));
    }

    #[test]
    fn branch_blocks_hit_count_and_found_count() {
        let mut branches = BranchBlocks::new();
        let b1 = &BranchData { line: 1, block: 0, branch: 1, taken: 1 };
        let b2 = &BranchData { line: 1, block: 0, branch: 2, taken: 0 };

        branches += b1;
        branches += b2;

        assert_eq!(branches.hit_count(), 1);
        assert_eq!(branches.found_count(), 2);
    }

    #[test]
    fn branches_hit_count_and_found_count() {
        let mut branches = Branches::new();
        branches += &BranchData { line: 1, block: 0, branch: 1, taken: 1 };
        branches += &BranchData { line: 1, block: 0, branch: 2, taken: 0 };

        assert_eq!(branches.hit_count(), 1);
        assert_eq!(branches.found_count(), 2);
    }
}
