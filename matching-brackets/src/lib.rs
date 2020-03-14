// algo from: https://www.geeksforgeeks.org/check-for-balanced-parentheses-in-an-expression/
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::<char>::new();

    let opening_brackets = ['(', '[', '{'];
    let closing_brackets = [')', ']', '}'];

    for c in string.chars() {
        if opening_brackets.contains(&c) {
            stack.push(c);
        } else if closing_brackets.contains(&c) {
            let popped = match stack.pop() {
                Some(n) => n,
                None => ' ', // This feels dirty
            };

            let should_be = match c {
                ')' => '(',
                ']' => '[',
                '}' => '{',
                _ => ' ',
            };

            if popped != should_be {
                return false;
            }
        }
    }
    stack.is_empty()
}
