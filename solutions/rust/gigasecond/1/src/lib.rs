use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {    
    let a = start + Duration::new(1000 * 1000000,0);
    a
}
