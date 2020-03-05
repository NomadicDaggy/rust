use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // Graphemes returns double-ended iterator, so we can
    // reverse it with rev()
    let mut iter = input.graphemes(true).rev();

    // output String
    let mut output = String::new();

    // Iterate through reversed chars
    while let Some(character) = iter.next() {
        output.push_str(character);
    }

    output
}
