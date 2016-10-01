use std::hash:: { Hash, Hasher };
use std::cmp:: { Eq, PartialEq };
use std::fmt:: { Display, Formatter, Result, Error };

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
#[derive(Debug, Eq)]
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

pub struct Branch {
    block: u32,
    branch: u32,
    taken: u32
}

#[cfg(test)]
mod tests {
    use branch:: { BranchUnit };
    use std::collections:: { HashMap };

    #[test]
    fn branch_unit() {
        let branch1 = BranchUnit(1, 1);
        let branch2 = BranchUnit(1, 2);
        assert!(branch1 != branch2, "branch1 = {}, branch2 = {}", branch1, branch2);

        let same_branch1 = BranchUnit(1, 1);
        let same_branch2 = BranchUnit(1, 1);
        assert!(same_branch1 == same_branch2, "branch1 = {}, branch2 = {}", same_branch1, same_branch2);
    }

    #[test]
    fn branch_unit_as_hash_key() {
        let mut container = HashMap::new();
        container.insert(BranchUnit(1, 1), 1);

        assert!( container.contains_key(&BranchUnit(1, 1)) );
    }
}
