// #[cfg(test)]
// mod tests {
//     use test_case::test_case;
//
//     #[test_case(-2, -4 ; "when both operands are negative")]
//     #[test_case(2,  4  ; "when both operands are positive")]
//     #[test_case(4,  2  ; "when operands are swapped")]
//     fn multiplication_tests(x: i8, y: i8) {
//         let actual = (x * y).abs();
//
//         assert_eq!(8, actual)
//     }
// }

// use rstest::rstest;
// #[rstest]
// #[case(0, 0)]
// #[case(1, 1)]
// #[case(2, 1)]
// #[case(3, 2)]
// #[case(4, 3)]
// fn fibonacci_test(#[case] input: u32, #[case] expected: u32) {
//     assert_eq!(expected, fibonacci(input))
// }
//
//
// // Top down approach / Recursive /Memoization
// fn fibonacci(n: u32) -> u32 {
//     match n {
//         0 => 0,
//         1 => 1,
//         _ => fibonacci(n - 1) + fibonacci(n - 2)
//     }
// }
//
//
//
