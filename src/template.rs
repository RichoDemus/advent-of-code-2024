use aoc_runner_derive::aoc;

#[aoc(dayX, part1)]
fn part1(input: &str) -> usize {
    todo!()
}

// #[aoc(dayX, part2)]
// fn part2(input: &str) -> usize {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

// #[test]
    // fn verify_part1() {
    //     let input = include_str!("../input/2024/dayX.txt");
    //     assert_eq!(part1(input), 0);
    // }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2024/dayX.txt");
    //     assert_eq!(part2(input), 0);
    // }

    #[test]
    fn part1_provided_example() {
        let _ = env_logger::builder()
            .filter_module("advent_of_code_2024", log::LevelFilter::Info)
            .try_init();
        let result = part1(
            r#""#,
        );

        assert_eq!(result, 0)
    }

    // #[test]
    // fn part2_provided_example() {
    // let _ = env_logger::builder()
    // .filter_module("advent_of_code_2024", log::LevelFilter::Info)
    // .try_init();
    //     let result = part2(
    //         r#""#,
    //     );
    //
    //     assert_eq!(result, 0)
    // }
}
