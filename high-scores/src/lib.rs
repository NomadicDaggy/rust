use std::cmp::Reverse;

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores: &scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        // cloned() is and iterator, that clones the elements
        // of its parent iterator.
        self.scores().last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        // max() works only on iterators
        self.scores().iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut vec = self.scores().to_vec();

        // Weird way to reverse sort
        vec.sort_by_key(|&num| Reverse(num));

        // Elegant way to cut off at 3rd element, inclusive.
        // If there are less elements than 3, it just returns all.
        // Also keeps it a Vec, not slice
        vec.truncate(3);

        vec
    }
}
