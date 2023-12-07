use std::{cmp::Ordering, collections::HashSet, fs};

#[derive(Clone, PartialOrd, PartialEq, Eq, Hash)]
struct Hand {
    cards: Vec<u32>,
    bid: usize,
    hand_type: usize, // 0 == high_card, 1 == one pair, 2 == two pair, 3 == three of a kind, 4 == full house, 5 == four of a kind, 6 == five of a kind
    line: String,
}

fn get_type_of_cards(cards: &Vec<u32>) -> usize {
    let set: HashSet<u32> = HashSet::from_iter(cards.clone());
    let vec_set: Vec<u32> = set.into_iter().collect();
    let occurences = vec_set
        .iter()
        .map(|set_number| cards.iter().filter(|&n| n == set_number).count())
        .collect::<Vec<usize>>();
    let max_occurences = *occurences.iter().max().unwrap();
    match vec_set.len() {
        1 | 2 => return max_occurences + 1,
        3 => return max_occurences,
        4 | 5 => return max_occurences - 1,
        _ => panic!("HashSet length does not match"),
    }
}

// 0 == high_card, 1 == one pair, 2 == two pair, 3 == three of a kind, 4 == full house, 5 == four of a kind, 6 == five of a kind
fn optimize_hand_type_with_jokers(hand: &Hand, jokers: usize) -> usize {
    match (hand.hand_type, jokers) {
        (_, 0) | (_, 5) => return hand.hand_type,
        (0, 1) => return 1,
        (1, 1) | (0, 2) | (1, 2) => return 3,
        (2, 1) => return 4,
        (3, 1) | (2, 2) => return 5,
        (5, 1) | (4, 2) => return 6,
        (_, 3) => return hand.hand_type + 2,
        (_, 4) => return 6,
        (_, _) => panic!("Optimize does not match ({},{jokers})", hand.hand_type),
    }
}

fn optimize_hand(hand: Hand) -> Hand {
    let jokers = hand.cards.iter().filter(|&n| *n == 0).count();
    let mut optimized_hand = hand;
    optimized_hand.hand_type = optimize_hand_type_with_jokers(&optimized_hand, jokers);
    optimized_hand
}

fn sort_hands(hands: Vec<Hand>) -> Vec<Hand> {
    let mut sorted_hands = hands;
    sorted_hands.sort_unstable_by_key(|hand| (hand.hand_type, hand.cards[0], hand.cards[1],hand.cards[2],hand.cards[3],hand.cards[4]));
    sorted_hands.reverse();
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
                    'A' => 15,
                    'K' => 14,
                    'Q' => 13,
                    'J' => {
                        if part_two {
                            return 0;
                        } else {
                            return 12;
                        }
                    }
                    'T' => 11,
                    number => number.to_digit(10).unwrap() + 1,
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

fn get_winnings(hands: Vec<Hand>) -> usize {
    let sorted_hands = sort_hands(hands);
    let num_hands = sorted_hands.len();
    let mut sum = 0;
    for i in 0..num_hands {
        sum += sorted_hands[i].bid * (num_hands - i)
    }
    sum
}

fn solve_part_one(input: &Vec<String>) -> usize {
    let hands: Vec<Hand> = parse_hands(input, false);
    get_winnings(hands)
}

fn solve_part_two(input: &Vec<String>) -> usize {
    let hands: Vec<Hand> = parse_hands(input, true);
    let optimized_hands = hands
        .iter()
        .map(|hand| optimize_hand(hand.clone()))
        .collect::<Vec<Hand>>();
    get_winnings(optimized_hands)
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
