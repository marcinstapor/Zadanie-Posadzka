use std::io::{self, Stdin};
use std::str::FromStr;

/// read_number reads single number (or other object that implements FromStr trait)
/// from given Stdin and parse it to given T.
fn read_number<T: FromStr>(stdin: &Stdin) -> Result<T, <T as FromStr>::Err> {
    let mut buffer = String::new();
    let _ = stdin.read_line(&mut buffer);
    buffer.trim().parse::<T>()
}

/// calc_remainder returns number of remainder floor tiles that are needed
/// to fill floor, which has got given width and height and number of floor
/// tiles placed equal to given quantity.
fn calc_remainder(width: i64, height: i64, quantity: i64) -> i64 {
    4 * width * height - quantity
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let a: i64 = read_number(&stdin)?;
    let b: i64 = read_number(&stdin)?;
    let n: i64 = read_number(&stdin)?;

    let ans = calc_remainder(a, b, n);
    println!("{}", ans);

    Ok(())
}
