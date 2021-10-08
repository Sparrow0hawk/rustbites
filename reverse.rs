pub fn reverse(input: &str) -> String {
    let result = input.chars().rev().collect::<String>();
    println!("{}", result);
    result
}

fn main() {
    let s = "World";

    reverse(s);
}

