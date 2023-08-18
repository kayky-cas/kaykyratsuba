fn add_str(n1: &str, n2: &str) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let mut n1: Vec<char> = n1.chars().collect();
    let mut n2: Vec<char> = n2.chars().collect();

    let max = n1.len().max(n2.len());

    for _ in 0..max {
        let first: u8 = n1.pop().unwrap_or('0') as u8 - b'0';
        let second: u8 = n2.pop().unwrap_or('0') as u8 - b'0';

        let mut sum: String = (first + second + carry).to_string().chars().rev().collect();

        carry = if sum.len() > 1 {
            sum.pop().unwrap() as u8 - b'0'
        } else {
            0
        };

        result.push(sum.chars().nth(0).unwrap());
    }

    if carry > 0 {
        result.push(carry.to_string().chars().nth(0).unwrap());
    }

    result
        .chars()
        .rev()
        .skip_while(|&x| result.len() != 1 && x == '0')
        .collect()
}

fn sub_str(n1: &str, n2: &str) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let mut n1: Vec<char> = n1.chars().collect();
    let mut n2: Vec<char> = n2.chars().collect();

    for _ in 0..n1.len() {
        let first = (n1.pop().unwrap_or('0') as u8 - b'0') as i8;
        let second = (n2.pop().unwrap_or('0') as u8 - b'0') as i8;

        let mut sub = first - second - carry;

        carry = if sub < 0 {
            sub += 10;
            1
        } else {
            0
        };

        result.push_str(&sub.to_string());
    }

    result
        .chars()
        .rev()
        .skip_while(|&x| result.len() != 1 && x == '0')
        .collect()
}

pub fn karatsuba<'a>(x: &'a str, y: &'a str) -> String {
    let mut max = x.max(y).len();

    if max < 2 {
        let x: usize = x.parse().unwrap_or(0);
        let y: usize = y.parse().unwrap_or(0);

        return (x * y).to_string();
    }

    if max % 2 != 0 {
        max += 1;
    }

    let mut x = x.to_owned();
    let mut y = y.to_owned();

    if x.len() < max {
        x = "0".repeat(max - x.len()) + &x;
    }

    if y.len() < max {
        y = "0".repeat(max - y.len()) + &y;
    }

    let half = max / 2;

    let (x1, x2) = x.split_at(half);
    let (y1, y2) = y.split_at(half);

    let mut a = karatsuba(x1, y1);
    let b = karatsuba(&add_str(x1, x2), &add_str(y1, y2));
    let c = karatsuba(x2, y2);

    let mut meiuca = sub_str(&b, &add_str(&a, &c));

    a.push_str(&"0".repeat(max));
    meiuca.push_str(&"0".repeat(half));

    add_str(&add_str(&a, &meiuca), &c)
}

#[cfg(test)]
mod tests {
    use crate::{add_str, karatsuba, sub_str};
    use num::integer::Roots;

    #[test]
    fn test_sub_str() {
        assert_eq!("90", sub_str("100", "10"));
    }

    #[test]
    fn test_add_str() {
        assert_eq!("90", add_str("010", "80"));

        const MAX: usize = 1_000;

        for x in 0..MAX {
            for y in 0..MAX {
                assert_eq!((x + y).to_string(), add_str(&x.to_string(), &y.to_string()))
            }
        }
    }

    #[test]
    fn test_karatsuba_0() {
        assert_eq!("0", karatsuba("0", "0"));
    }

    #[test]
    fn test_karatsuba_1() {
        assert_eq!("100", karatsuba("10", "10"));
    }

    #[test]
    fn test_karatsuba_2() {
        assert_eq!("81", karatsuba("3", "27"));
    }

    #[test]
    fn test_karatsuba_final() {
        let max: u128 = u128::MAX.sqrt();

        for x in 0..max {
            for y in 0..max {
                assert_eq!(
                    (x * y).to_string(),
                    karatsuba(&x.to_string(), &y.to_string())
                );
            }
        }
    }
}
