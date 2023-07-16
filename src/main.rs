fn fizzbuzz(x: i32) -> String {
    match ((x % 3) == 0, (x % 5) == 0) {
        (true, true) => "Fizzbuzz".to_string(),
        (true, _) => "Fizz".to_string(),
        (_, true) => "Buzz".to_string(),
        _ => x.to_string(),
    }
}

fn main() {
    (1..=15).map(fizzbuzz).for_each(|x| println!("{x}"))
}

#[test]
fn test() {
    let test_target: Vec<String> = (1..=15).map(fizzbuzz).collect();

    assert_eq!(
        test_target,
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "Fizzbuzz"
        ]
    );
}