pub fn build_proverb(list: &[&str]) -> String {
    let mut string_list = Vec::<String>::new();

    if list.is_empty() {
        return String::new();
    }

    for i in 0..list.len() - 1 {
        // -1 because last line will be special
        let line = format!("For want of a {} the {} was lost.", list[i], list[i + 1]);
        string_list.push(line);
    }

    string_list.push(format!("And all for the want of a {}.", list[0]));

    string_list.join("\n")
}
