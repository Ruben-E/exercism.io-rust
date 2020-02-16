extern crate chrono;
use chrono::*;

const GIGASECOND: i64 = 1_000_000_000;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    return start.checked_add_signed(Duration::seconds(GIGASECOND)).expect("Overflow")
}
