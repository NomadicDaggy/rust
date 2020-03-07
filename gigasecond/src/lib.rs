use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // Not entirely sure that I'm allowed to do this as it seems too easy
    // ...but the exercise did say to check what operations can be performed
    // on DateTime in the chrono crate. So I take it as a hint to not
    // re-invent the wheel and use all tools at my disposal :D.
    start + Duration::seconds(1_000_000_000)
}
