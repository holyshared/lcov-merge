use std::io;
use std::fmt:: { Display, Formatter, Result };
use std::collections::btree_map:: { BTreeMap };
use lcov_parser:: { BranchData };
use merger::ops:: { Merge };
use record:: { RecordWrite };
use report::summary:: { Summary };
use report::attribute:: { LineNumber, ExecutionCount };
use report::counter:: { Hit, HitFoundCounter, FoundCounter, HitCounter };

/// Units of the branch
///
/// # Examples
///
/// ```
/// use lcov_merge::branch::BranchUnit;
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
#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct BranchUnit(u32, u32);

impl BranchUnit {
    pub fn new(block: u32, branch: u32) -> BranchUnit {
        BranchUnit(block, branch)
    }
    pub fn block(&self) -> u32 {
        self.0
    }
    pub fn branch(&self) -> u32 {
        self.1
    }
}

impl Display for BranchUnit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BranchBlocks {
    blocks: BTreeMap<BranchUnit, ExecutionCount>
}

impl BranchBlocks {
    pub fn new() -> Self {
        BranchBlocks {
            blocks: BTreeMap::new()
        }
    }
}

impl_summary!(BranchBlocks, blocks<BranchUnit, ExecutionCount>);


impl HitCounter for BranchBlocks {
    fn hit_count(&self) -> usize {
        self.iter()
            .filter(|&(_, taken)| taken.is_hit())
            .count()
    }
}

impl FoundCounter for BranchBlocks {
    fn found_count(&self) -> usize {
        self.blocks.len()
    }
}

impl HitFoundCounter for BranchBlocks {}



impl<'a> Merge<&'a BranchData> for BranchBlocks {
    fn merge(&mut self, data: &'a BranchData) {
        let unit = BranchUnit::new(data.block, data.branch);

        if !self.blocks.contains_key(&unit) {
            self.blocks.insert(unit, data.taken);
            return;
        }
        let mut block_count = self.blocks.get_mut(&unit).unwrap();
        *block_count += data.taken;
    }
}

impl<'a> Merge<&'a BranchBlocks> for BranchBlocks {
    fn merge(&mut self, other: &'a BranchBlocks) {
        for (unit, taken) in other.iter() {
            if !self.blocks.contains_key(unit) {
                self.blocks.insert(unit.clone(), taken.clone());
                continue;
            }
            let mut block = self.blocks.get_mut(unit).unwrap();
            *block += *taken;
        }
    }
}




#[derive(Debug, Clone)]
pub struct Branches {
    branches: BTreeMap<LineNumber, BranchBlocks>
}

impl Branches {
    pub fn new() -> Self {
        Branches {
            branches: BTreeMap::new()
        }
    }
}

impl AsRef<BTreeMap<LineNumber, BranchBlocks>> for Branches {
    fn as_ref(&self) -> &BTreeMap<LineNumber, BranchBlocks> {
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


impl_summary!(Branches, branches<LineNumber, BranchBlocks>);


impl RecordWrite for Branches {
    fn write_records<T: io::Write>(&self, output: &mut T) -> io::Result<()> {
        write!(output, "{}", self)
    }
}

impl Display for Branches {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.is_empty() {
            return Ok(());
        }
        for (line_number, blocks) in self.iter() {
            for (unit, taken) in blocks.iter() {
                try!(writeln!(f, "BRDA:{},{},{},{}",
                    line_number, unit.block(), unit.branch(), taken));
            }
        }
        try!(writeln!(f, "BRF:{}", self.found_count()));
        try!(writeln!(f, "BRH:{}", self.hit_count()));
        Ok(())
    }
}


impl<'a> Merge<&'a Branches> for Branches {
    fn merge(&mut self, other: &'a Branches) {
        for (line_number, other_blocks) in other.iter() {
            if !self.branches.contains_key(line_number) {
                self.branches.insert(line_number.clone(), other_blocks.clone());
                continue;
            }
            let mut blocks = self.branches.get_mut(line_number).unwrap();
            blocks.merge(other_blocks);
        }
    }
}

impl<'a> Merge<&'a BranchData> for Branches {
    fn merge(&mut self, data: &'a BranchData) {
        if self.branches.contains_key(&data.line) {
            let mut blocks = self.branches.get_mut(&data.line).unwrap();
            blocks.merge(data);
        } else {
            let blocks = {
                let mut blocks = BranchBlocks::new();
                blocks.merge(data);
                blocks
            };
            self.branches.insert(
                data.line.clone(),
                blocks
            );
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections:: { HashMap };
    use lcov_parser:: { BranchData };
    use merger::ops::*;
    use report::branch:: { BranchUnit, Branches, BranchBlocks };
    use report::summary:: { Summary };
    use report::counter:: { FoundCounter, HitCounter };

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

        branches.merge(b1);
        branches.merge(b2);

        assert_eq!(branches.get(&BranchUnit::new(0, 1)), Some(&2));
    }

    #[test]
    fn append_branches() {
        let mut branches = BranchBlocks::new();
        let b1 = &BranchData { line: 1, block: 0, branch: 1, taken: 1 };
        let b2 = &BranchData { line: 1, block: 0, branch: 1, taken: 1 };

        branches.merge(b1);
        branches.merge(b2);

        let cloned_branches = branches.clone();

        branches.merge(&cloned_branches);

        assert_eq!(branches.get(&BranchUnit::new(0, 1)), Some(&4));
    }

    #[test]
    fn branch_blocks_hit_count_and_found_count() {
        let mut branches = BranchBlocks::new();
        let b1 = &BranchData { line: 1, block: 0, branch: 1, taken: 1 };
        let b2 = &BranchData { line: 1, block: 0, branch: 2, taken: 0 };

        branches.merge(b1);
        branches.merge(b2);

        assert_eq!(branches.hit_count(), 1);
        assert_eq!(branches.found_count(), 2);
    }

    #[test]
    fn branches_hit_count_and_found_count() {
        let mut branches = Branches::new();
        branches.merge(&BranchData { line: 1, block: 0, branch: 1, taken: 1 });
        branches.merge(&BranchData { line: 1, block: 0, branch: 2, taken: 0 });

        assert_eq!(branches.hit_count(), 1);
        assert_eq!(branches.found_count(), 2);
    }
}
