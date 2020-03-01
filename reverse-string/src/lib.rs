use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // Naive solution could be to push input onto a stack structure,
    // then just pop it off into returnable String]

    // Essentially this takes in &str, splits it into unicode characters
    // which might aswell be several bytes in length each, and puts them
    // into a Vec as &str
    let mut stack = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();

    // output String
    let mut output = String::new();

    // Pop chars off in reverse order
    // The "while let Some" syntax seems very weird to me
    while let Some(character) = stack.pop() {
        output.push_str(character);
    }

    output
}
