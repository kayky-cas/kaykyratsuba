fn add_str(n1: &str, n2: &str) -> String {
    let mut n1: Vec<char> = n1.chars().collect();
    let mut n2: Vec<char> = n2.chars().collect();

    let mut carry = 0;
    let mut result = String::new();

    while !n1.is_empty() || !n2.is_empty() {
        let first: u8 = n1.pop().unwrap_or('0') as u8 - b'0';
        let second: u8 = n2.pop().unwrap_or('0') as u8 - b'0';

        let mut sum: String = (first + second + carry).to_string().chars().rev().collect();

        carry = if sum.len() > 1 {
            sum.pop().unwrap() as u8 - b'0'
        } else {
            0
        };

        result.push(sum.chars().next().unwrap());
    }

    if carry > 0 {
        result.push(carry.to_string().chars().next().unwrap());
    }

    let result: String = result.chars().rev().skip_while(|&x| x == '0').collect();

    if result.is_empty() {
        "0".to_owned()
    } else {
        result
    }
}

#[macro_export]
macro_rules! shift_decimal {
    ($x:expr) => {
        "0".repeat($x)
    };
}

pub fn karatsuba<'a>(x: &'a str, y: &'a str) -> String {
    if x.chars().all(|x| x == '0') || y.chars().all(|x| x == '0') {
        return "0".to_owned();
    }

    let mut max = x.len().max(y.len());

    if max < 2 {
        let x = x.chars().next().unwrap_or('0') as u8 - b'0';
        let y = y.chars().next().unwrap_or('0') as u8 - b'0';

        return (x * y).to_string();
    }

    while max % 3 != 0 {
        max += 1;
    }

    let x = if x.len() < max {
        shift_decimal!(max - x.len()) + x
    } else {
        x.to_owned()
    };

    let y = if y.len() < max {
        shift_decimal!(max - y.len()) + y
    } else {
        y.to_owned()
    };

    let tird = max / 3;

    let x: Vec<String> = x
        .chars()
        .collect::<Vec<char>>()
        .chunks(tird)
        .map(|x| x.iter().collect::<String>())
        .collect();

    let y: Vec<String> = y
        .chars()
        .collect::<Vec<char>>()
        .chunks(tird)
        .map(|x| x.iter().collect::<String>())
        .collect();

    let mut result = String::new();

    for i in 0..3 {
        for j in 0..3 {
            let curr = karatsuba(&x[i], &y[j]) + &shift_decimal!(tird * (4 - i - j));
            result = add_str(&result, &curr);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{add_str, karatsuba};
    use num::integer::Roots;

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
    fn test_karatsuba_3() {
        assert_eq!("10000", karatsuba("500", "20"));
    }

    #[test]
    fn test_karatsuba_final() {
        let max = 1000000.sqrt();

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
