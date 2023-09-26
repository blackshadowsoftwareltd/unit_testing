pub fn validate(v: i32) -> Result<(), ()> {
    if v < 0 {
        Err(())
    } else {
        Ok(())
    }
}
