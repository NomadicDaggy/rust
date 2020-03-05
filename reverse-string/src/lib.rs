use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // Graphemes returns a double-ended iterator, so we can
    // reverse it with rev()
    let iter = input.graphemes(true).rev();

    // Iterators can be collected into various collections,
    // one of which is String
    iter.collect::<String>()
}
