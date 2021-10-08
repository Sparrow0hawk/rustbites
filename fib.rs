use std::env;

pub fn main() {

    let args: Vec<String> = env::args().collect();

    let n_arg = args[1].parse().unwrap();

    let mut fibvec = vec![1, 1];

    for i in 2..n_arg {
        let next_val = fibvec[i - 1] + fibvec[i - 2];
        fibvec.push(next_val)
    }
    println!("{:?}", fibvec)
}
