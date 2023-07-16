fn main() {
    (1..=15).for_each(|x| match ((x % 3) == 0, (x % 5) == 0) {
        (true, true) => println!("Fizzbuzz"),
        (true, _) => println!("Fizz"),
        (_, true) => println!("Buzz"),
        _ => println!("{x}"),
    })
}
