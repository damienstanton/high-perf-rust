fn main() {
    // use zero-cost abstractions where possible
    let arr = [1, 2, 3, 4, 5, 10];
    let sum = arr.iter().filter(|n| *n % 2 == 0).fold(0, |acc, n| acc + n);
    debug_assert_eq!(sum, 16);

    // smoke test
    println!("OK");
}
