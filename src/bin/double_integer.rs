/* https://www.codewars.com/kata/53ee5429ba190077850011d4 */

fn double_integer(n: i32) -> i32 {
    n * 2
}

fn main() {
    let x = double_integer(1);

    println!("{}", x)
}

#[cfg(test)]
mod tests {
    use super::double_integer;

    #[test]
    fn sample_tests() {
        assert_eq!(double_integer(1), 2);
        assert_eq!(double_integer(5), 10);
        assert_eq!(double_integer(10), 20);
        assert_eq!(double_integer(20), 40);
        assert_eq!(double_integer(40), 80);
    }
}
