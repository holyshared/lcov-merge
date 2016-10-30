pub trait HitCounter {
    fn hit_count(&self) -> usize;
}

pub trait FoundCounter {
    fn found_count(&self) -> usize;
}

pub trait HitFoundCounter:HitCounter + FoundCounter {
}
