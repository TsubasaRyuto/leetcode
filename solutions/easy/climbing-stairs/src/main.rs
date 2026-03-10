fn main() {
    println!("Hello, world!");
}

struct Solution {}
impl Solution {
    fn permutation(n: i32, r: i32) -> i32 {
        let mut result = 1;
        for i in (n - r + 1..=n).rev() {
            result *= i;
        }

        result
    }

    fn factorial(n: i32) -> i32 {
        let mut result = 1;
        for i in 2..=n {
            result *= i
        }

        result as i32
    }

    pub fn climb_stairs(n: i32) -> i32 {
        let two_step_count = n / 2;
        let mut result = 0;

        for i in 1..=two_step_count {
            result += (Solution::permutation((n - i), i) / Solution::factorial(i));
        }

        result + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs_case1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test_climb_stairs_case2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn test_climb_stairs_case3() {
        assert_eq!(Solution::climb_stairs(6), 13);
    }

    #[test]
    fn test_climb_stairs_case4() {
        assert_eq!(Solution::climb_stairs(1), 1);
    }
}

// Goal
// return the number of combinations of ways to climb to the top
//
// // input = 2
// 1 step + 1 step
// 2 step
// return 2,
//
//
// 1. Loop while 1..=n/2
// -> Iterate from 1 up to n/2.
// 2. Add the compination result, ${}_{n-i} C_{i}$.
// -> Sum the combinations  ${}_{n-i}C_i$
// 3. return result + 1
// -> Return the total sum plus 1.
// 4. why it plus oen at last, it is all one case.
// -> Note: The final "+1" represents the case where only 1-step climbs are used.
