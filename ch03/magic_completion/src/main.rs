fn main() {
    let bananas = 5.0;
    let apes = 2.0;

    println!(
        "bananas={bananas} apps={apes}, bananas_per_ape={}",
        bananas / apes
    );
}

#[test]
fn feature() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_name() {}
}
