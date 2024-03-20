use game_helper_v2::board::Board;

pub struct RangeSet {
    elements: Vec<Board>,
    ranges: Vec<(u64, u64)>,
}

impl RangeSet {
    pub fn new() -> Self {
        RangeSet {
            elements: Vec::new(),
            ranges: Vec::new(),
        }
    }

    pub fn insert(&mut self, item: Board) {
        let index = self.elements.binary_search(&item).unwrap_or_else(|x| x);
        self.elements.insert(index, item);
        self.update_ranges();
    }

    pub fn contains(&self, item: u64) -> bool {
        self.ranges
            .binary_search_by(|&(start, end)| {
                if item < start {
                    std::cmp::Ordering::Greater
                } else if item > end {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .is_ok()
    }

    fn update_ranges(&mut self) {
        self.ranges.clear();
        let mut start: u64 = self.elements[0].0;
        let mut end: u64 = start;
        for &item in &self.elements[1..] {
            if item.0 == end + 1 {
                end = item.0;
            } else {
                self.ranges.push((start, end));
                start = item.0;
                end = start;
            }
        }
        self.ranges.push((start, end));
    }
}
