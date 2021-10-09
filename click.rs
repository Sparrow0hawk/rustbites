#[derive(Debug)]
struct Cli {
    pattern: String,
}

fn main() {
    let big_string = std::env::args().nth(1).expect("No string given");

    let args = Cli {
        pattern: big_string
    };

    let split = args.pattern.split("");

    for s in split {
        println!("{}", s)
    }
}
