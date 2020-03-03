fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn verse(n: u32) -> String {
    let nm = String::from("no more");
    let bsob = String::from("bottles of beer");
    let bob = String::from("bottle of beer");
    let otw = String::from("on the wall");
    let take = String::from("Take one down and pass it around, ");
    let store = String::from("Go to the store and buy some more, ");
    let bow = String::from(" bottles of beer on the wall, ");

    if n == 0 {
        return format!(
            "{} {} {}, {} {}.\n{}99 {} {}.\n",
            capitalize(&nm),
            &bsob,
            &otw,
            &nm,
            &bsob,
            &store,
            &bsob,
            &otw
        );
    } else if n == 1 {
        return format!("1 {} {}, 1 {}.\nTake it down and pass it around, no more bottles of beer on the wall.\n", &bob, &otw, &bob);
    } else if 1 < n && n < 99 {
        return format!("{}{}{}{}.\n{}", n, &bow, n, &bob, &take, (n-1).to_string())
    }

    nm
}

pub fn sing(start: u32, end: u32) -> String {
    unimplemented!("sing verses {} to {}, inclusive", start, end)
}
