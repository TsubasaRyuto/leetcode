fn main() {
    println!("Hello, world!");
}

struct Solution {}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_int: i64 = a.parse().unwrap();
        let b_int: i64 = b.parse().unwrap();
        let sum = a_int + b_int;

        println!("{}", sum);

        let mut result = String::new();
        let mut before_val = 0;
        for val in sum.to_string().chars().rev() {
            if before_val == 1 {
                if val == '2' {
                    result += "1";
                    before_val = 1;
                    continue;
                }

                if val == '1' {
                    result += "0";
                    before_val = 1;
                    continue;
                }

                if val == '0' {
                    result += "1";
                    before_val = 0;
                    continue;
                }
            }

            if before_val == 0 {
                if val == '2' {
                    result += "0";
                    before_val = 1;
                    continue;
                }

                if val == '1' {
                    result += "1";
                    before_val = 0;
                    continue;
                }

                if val == '0' {
                    result += "0";
                    before_val = 0;
                    continue;
                }
            }
        }

        if before_val == 1 {
            result += "1";
        }

        let result: String = result.chars().rev().collect();

        result
    }
}

// 1. Pseudo-Conversion to a Numeric Value
//    interpret a binary string as if it were a decimal number(e.g. "11")
//
// 2. Batch Addition
//    Perform adition treating the values as decimal numbers.
//    Create a number where each digit represents the sum of the corresponding digits
//
// 3. Digit-by-Digit "binary Conversion" Adjustment
//   Starting from the lowest digit, check each position.
//   Whenever a digits is "2" or there is acarry, rewrite it according to binary rules(i.e., 2 -> 0 with a carry of 1 to the next higher digit), updating one character at a time.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary_case_1() {
        assert_eq!(
            Solution::add_binary("111".to_string(), "1".to_string()),
            "1000"
        )
    }

    #[test]
    fn test_add_binary_case_2() {
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101"
        )
    }

    #[test]
    fn test_add_binary_case_3() {
        assert_eq!(
            Solution::add_binary("0000".to_string(), "11111111".to_string()),
            "11111111"
        )
    }

    #[test]
    fn test_add_binary_case_4() {
        assert_eq!(
            Solution::add_binary("0001".to_string(), "11111111".to_string()),
            "100000000"
        )
    }

    #[test]
    fn test_add_binary_case_5() {
        assert_eq!(
            Solution::add_binary("1110110101".to_string(), "1110111011".to_string()),
            "11101110000"
        )
    }
}
