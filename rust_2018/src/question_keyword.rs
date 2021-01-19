fn main() -> Result<(), usize> {
    let array = [12, 19, 27];
    let found = array.binary_search(&19)?;
    println!("Found {}", found);
    let found = array.binary_search(&20)?;
    println!("Found {}", found);
    Ok(())
}
