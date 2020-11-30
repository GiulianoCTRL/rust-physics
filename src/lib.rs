pub struct Velocity;

// TODO: Everything
// Also, figure out a way to use any number type or even create number type for different measurement units
// e.g. distance, time, weight and so on


impl Velocity {
    /// Get avg velocity
    pub fn from_time_distance(s: i64, t: i64) -> i64 {
        s / t
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
