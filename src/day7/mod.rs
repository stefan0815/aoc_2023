use std::{cmp::Ordering, collections::HashSet, fs};

#[derive(Clone, PartialOrd, PartialEq, Eq, Hash)]
struct Hand {
    cards: Vec<u32>,
    bid: usize,
    hand_type: usize, // 0 == high_card, 1 == one pair, 2 == two pair, 3 == three of a kind, 4 == full house, 5 == four of a kind, 6 == five of a kind
    line: String,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            for i in 0..self.cards.len() {
                if self.cards[i] == other.cards[i] {
                    continue;
                }
                return self.cards[i].cmp(&other.cards[i]);
            }
            return Ordering::Equal;
        }
        self.hand_type.cmp(&other.hand_type)
    }
}

// full house or four of a kind
fn check_hashset_two(cards: &Vec<u32>, set: &HashSet<u32>) -> usize {
    let vec_set = set.into_iter().map(|n| *n).collect::<Vec<u32>>();
    match cards.iter().filter(|&n| *n == vec_set[0]).count() {
        1 | 4 => return 5,
        2 | 3 => return 4,
        _ => panic!("check_hashset_2 does not match"),
    }
}

// two pair, three of a kind
fn check_hashset_three(cards: &Vec<u32>, vec_set: &Vec<u32>) -> usize {
    if vec_set.len() == 0 {
        return 2;
    }
    match cards.iter().filter(|&n| *n == vec_set[0]).count() {
        3 => return 3,
        1 | 2 => {
            return check_hashset_three(
                cards,
                &vec_set.iter().skip(1).map(|n| *n).collect::<Vec<u32>>(),
            )
        }
        _ => panic!("check_hashset_2 does not match"),
    }
}

fn get_type_of_cards(cards: &Vec<u32>) -> usize {
    let set: HashSet<u32> = HashSet::from_iter(cards.clone());
    match set.len() {
        1 => return 6,
        2 => return check_hashset_two(cards, &set),
        3 => return check_hashset_three(cards, &set.into_iter().collect::<Vec<u32>>()),
        4 => return 1,
        5 => return 0,
        _ => panic!("HashSet length does not match"),
    }
}

// 0 == high_card, 1 == one pair, 2 == two pair, 3 == three of a kind, 4 == full house, 5 == four of a kind, 6 == five of a kind
fn optimize_hand_type_with_one_joker(hand: &Hand) -> usize{
    match hand.hand_type {
        0 => return 1,
        1 => return 3,
        2 => return 4,
        3 => return 5,
        5 => return 6,
        _ => panic!("One joker does not match")
    }
}

// 0 == high_card, 1 == one pair, 2 == two pair, 3 == three of a kind, 4 == full house, 5 == four of a kind, 6 == five of a kind
fn optimize_hand_type_with_two_joker(hand: &Hand) -> usize{
    match hand.hand_type {
        1 => return 3,
        2 => return 5,
        4 => return 6,
        _ => panic!("Two jokers do not match")
    }
}

// 0 == high_card, 1 == one pair, 2 == two pair, 3 == three of a kind, 4 == full house, 5 == four of a kind, 6 == five of a kind
fn optimize_hand_type_with_three_joker(hand: &Hand) -> usize{
    match hand.hand_type {
        3 => return 5,
        4 => return 6,
        _ => panic!("Three jokers do not match")
    }
}

fn optimize_hand(hand: Hand) -> Hand {
    let num_jokers = hand.cards.iter().filter(|&n| *n == 0).count();
    let mut optimized_hand = hand;
    match num_jokers {
        0 | 5 => return optimized_hand,
        1 => {
            optimized_hand.hand_type = optimize_hand_type_with_one_joker(&optimized_hand);
            return optimized_hand;
        },
        2 => {
            optimized_hand.hand_type = optimize_hand_type_with_two_joker(&optimized_hand);
            return optimized_hand;
        }
        3 => {
            optimized_hand.hand_type = optimize_hand_type_with_three_joker(&optimized_hand);
            return optimized_hand;
        }
        4 => {
            optimized_hand.hand_type = 6;
            return optimized_hand;
        }
        _ => panic!("Too many jokers"),
    }
}

fn sort_hands(hands: Vec<Hand>) -> Vec<Hand> {
    let mut sorted_hands = hands;
    sorted_hands.sort_by(|a, b| b.cmp(&a));
    sorted_hands
}

fn parse_hands(input: &Vec<String>, part_two: bool) -> Vec<Hand> {
    input
        .iter()
        .map(|line| {
            let split = line
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            let cards = split[0]
                .chars()
                .map(|char| match char {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => {
                        if part_two {
                            return 0;
                        } else {
                            return 11;
                        }
                    }
                    'T' => {
                        if part_two {
                            return 11;
                        } else {
                            return 10;
                        }
                    }
                    number => {
                        if part_two {
                            return number.to_digit(10).unwrap() + 1;
                        } else {
                            return number.to_digit(10).unwrap();
                        }
                    }
                })
                .collect::<Vec<u32>>();
            Hand {
                cards: cards.clone(),
                bid: split[1].parse::<usize>().unwrap(),
                hand_type: get_type_of_cards(&cards),
                line: line.to_string(),
            }
        })
        .collect()
}

fn solve_part_one(input: &Vec<String>) -> usize {
    let hands: Vec<Hand> = parse_hands(input, false);
    let sorted_hands = sort_hands(hands);
    let num_hands = sorted_hands.len();
    let mut sum = 0;
    for i in 0..num_hands {
        // println!(
        //     "{i}: {} type: {}, bid: {}, value: {}, rank: {}",
        //     sorted_hands[i].line,
        //     sorted_hands[i].hand_type,
        //     sorted_hands[i].bid,
        //     sorted_hands[i].bid * (num_hands - i),
        //     num_hands - i
        // );
        sum += sorted_hands[i].bid * (num_hands - i)
    }
    sum
}

fn solve_part_two(input: &Vec<String>) -> usize {
    let hands: Vec<Hand> = parse_hands(input, true);
    let optimized_hands = hands
        .iter()
        .map(|hand| optimize_hand(hand.clone()))
        .collect::<Vec<Hand>>();
    let sorted_hands = sort_hands(optimized_hands);
    let num_hands = sorted_hands.len();
    let mut sum = 0;
    for i in 0..num_hands {
        // println!(
        //     "{i}: {} type: {}, bid: {}, value: {}, rank: {}",
        //     sorted_hands[i].line,
        //     sorted_hands[i].hand_type,
        //     sorted_hands[i].bid,
        //     sorted_hands[i].bid * (num_hands - i),
        //     num_hands - i
        // );
        sum += sorted_hands[i].bid * (num_hands - i)
    }
    sum
}

fn get_input(file: &str) -> Vec<String> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let groups: Vec<String> = input
        .split("\r\n")
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    return groups;
}

pub fn solver() {
    let input = get_input("./src/day7/input.txt");
    let sum_part_one = solve_part_one(&input);
    println!("Part 1: {sum_part_one}");
    let sum_part_two = solve_part_two(&input);
    println!("Part 2: {sum_part_two}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test]
    fn day7_example_input_part_one() {
        let input = get_input("./src/day7/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(6440, sum_part_one);
    }

    #[test]
    fn day7_input_part_one() {
        let input = get_input("./src/day7/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(241344943, sum_part_one);
    }

    #[test]
    fn day7_example_input_part_two() {
        let input = get_input("./src/day7/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(5905, sum_part_two);
    }

    #[test]
    fn day7_input_part_two() {
        let input = get_input("./src/day7/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(243101568, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day7/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day7/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
