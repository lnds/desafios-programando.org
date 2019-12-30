use poker::manos_ganadoras;
use std::collections::HashSet;

fn hs_from<'a>(input: &[&'a str]) -> HashSet<&'a str> {
    let mut hs = HashSet::new();
    for item in input.iter() {
        hs.insert(*item);
    }
    hs
}

/// Test that the expected output is produced from the given input
/// using the `manos_ganadoras` function.
///
/// Note that the output can be in any order. Here, we use a HashSet to
/// abstract away the order of outputs.
fn test<'a, 'b>(input: &[&'a str], expected: &[&'b str]) {
    assert_eq!(
        hs_from(&manos_ganadoras(input).expect("This test should produce Some value",)),
        hs_from(expected)
    )
}

#[test]
fn test_single_hand_always_wins() {
    test(&["4P 5P 7C 8D JT"], &["4P 5P 7C 8D JT"])
}

#[test]
#[ignore]
fn test_highest_card_of_all_hands_wins() {
    test(
        &["4D 5P 6P 8D 3T", "2P 4T 7P 9C 10C", "3P 4P 5D 6C JC"],
        &["3P 4P 5D 6C JC"],
    )
}

#[test]
#[ignore]
fn test_a_tie_has_multiple_winners() {
    test(
        &[
            "4D 5P 6P 8D 3T",
            "2P 4T 7P 9C 10C",
            "3P 4P 5D 6C JC",
            "3C 4C 5T 6T JD",
        ],
        &["3P 4P 5D 6C JC", "3C 4C 5T 6T JD"],
    )
}

#[test]
#[ignore]
fn test_high_card_can_be_low_card_in_an_otherwise_tie() {
    // multiple hands with the same high cards, tie compares next highest ranked,
    // down to last card
    test(&["3P 5C 6P 8D 7C", "2P 5D 6D 8T 7P"], &["3P 5C 6P 8D 7C"])
}

#[test]
#[ignore]
fn test_one_pair_beats_high_card() {
    test(&["4P 5C 6T 8D KC", "2P 4C 6P 4D JC"], &["2P 4C 6P 4D JC"])
}

#[test]
#[ignore]
fn test_highest_pair_wins() {
    test(&["4P 2C 6P 2D JC", "2P 4C 6T 4D JD"], &["2P 4C 6T 4D JD"])
}

#[test]
#[ignore]
fn test_two_pairs_beats_one_pair() {
    test(&["2P 8C 6P 8D JC", "4P 5C 4T 8T 5T"], &["4P 5C 4T 8T 5T"])
}

#[test]
#[ignore]
fn test_two_pair_ranks() {
    // both hands have two pairs, highest ranked pair wins
    test(&["2P 8C 2D 8D 3C", "4P 5C 4T 8P 5D"], &["2P 8C 2D 8D 3C"])
}

#[test]
#[ignore]
fn test_two_pairs_second_pair_cascade() {
    // both hands have two pairs, with the same highest ranked pair,
    // tie goes to low pair
    test(&["2P QP 2T QD JC", "JD QC JP 8D QT"], &["JD QC JP 8D QT"])
}

#[test]
#[ignore]
fn test_two_pairs_last_card_cascade() {
    // both hands have two identically ranked pairs,
    // tie goes to remaining card (kicker)
    test(&["JD QC JP 8D QT", "JP QP JT 2D QD"], &["JD QC JP 8D QT"])
}

#[test]
#[ignore]
fn test_three_of_a_kind_beats_two_pair() {
    test(&["2P 8C 2C 8D JC", "4P 5C 4T 8P 4C"], &["4P 5C 4T 8P 4C"])
}

#[test]
#[ignore]
fn test_three_of_a_kind_ranks() {
    //both hands have three of a kind, tie goes to highest ranked triplet
    test(&["2P 2C 2T 8D JC", "4P AC AP 8T AD"], &["4P AC AP 8T AD"])
}

#[test]
#[ignore]
fn test_three_of_a_kind_cascade_ranks() {
    // with multiple decks, two players can have same three of a kind,
    // ties go to highest remaining cards
    test(&["4P AC AP 7T AD", "4P AC AP 8T AD"], &["4P AC AP 8T AD"])
}

#[test]
#[ignore]
fn test_straight_beats_three_of_a_kind() {
    test(&["4P 5C 4T 8D 4C", "3P 4D 2P 6D 5T"], &["3P 4D 2P 6D 5T"])
}

#[test]
#[ignore]
fn test_aces_can_end_a_straight_high() {
    // aces can end a straight (10 J Q K A)
    test(&["4P 5C 4T 8D 4C", "10D JC QP KD AT"], &["10D JC QP KD AT"])
}

#[test]
#[ignore]
fn test_aces_can_end_a_straight_low() {
    // aces can start a straight (A 2 3 4 5)
    test(&["4P 5C 4T 8D 4C", "4D AC 3P 2D 5T"], &["4D AC 3P 2D 5T"])
}

#[test]
#[ignore]
fn test_straight_cascade() {
    // both hands with a straight, tie goes to highest ranked card
    test(&["4P 6T 7P 8D 5C", "5P 7C 8P 9D 6C"], &["5P 7C 8P 9D 6C"])
}

#[test]
#[ignore]
fn test_straight_scoring() {
    // even though an ace is usually high, a 5-high straight is the lowest-scoring straight
    test(&["2C 3T 4D 5D 6C", "4P AC 3P 2D 5C"], &["2C 3T 4D 5D 6C"])
}

#[test]
#[ignore]
fn test_flush_beats_a_straight() {
    test(&["4T 6C 7D 8D 5C", "2P 4P 5P 6P 7P"], &["2P 4P 5P 6P 7P"])
}

#[test]
#[ignore]
fn test_flush_cascade() {
    // both hands have a flush, tie goes to high card, down to the last one if necessary
    test(&["4C 7C 8C 9C 6C", "2P 4P 5P 6P 7P"], &["4C 7C 8C 9C 6C"])
}

#[test]
#[ignore]
fn test_full_house_beats_a_flush() {
    test(&["3C 6C 7C 8C 5C", "4P 5T 4T 5D 4C"], &["4P 5T 4T 5D 4C"])
}

#[test]
#[ignore]
fn test_full_house_ranks() {
    // both hands have a full house, tie goes to highest-ranked triplet
    test(&["4C 4P 4D 9P 9D", "5C 5P 5D 8P 8D"], &["5C 5P 5D 8P 8D"])
}

#[test]
#[ignore]
fn test_full_house_cascade() {
    // with multiple decks, both hands have a full house with the same triplet, tie goes to the pair
    test(&["5C 5P 5D 9P 9D", "5C 5P 5D 8P 8D"], &["5C 5P 5D 9P 9D"])
}

#[test]
#[ignore]
fn test_four_of_a_kind_beats_full_house() {
    test(&["4P 5C 4D 5D 4C", "3P 3C 2P 3D 3T"], &["3P 3C 2P 3D 3T"])
}

#[test]
#[ignore]
fn test_four_of_a_kind_ranks() {
    // both hands have four of a kind, tie goes to high quad
    test(&["2P 2C 2T 8D 2D", "4P 5C 5P 5D 5T"], &["4P 5C 5P 5D 5T"])
}

#[test]
#[ignore]
fn test_four_of_a_kind_cascade() {
    // with multiple decks, both hands with identical four of a kind, tie determined by kicker
    test(&["3P 3C 2P 3D 3T", "3P 3C 4P 3D 3T"], &["3P 3C 4P 3D 3T"])
}

#[test]
#[ignore]
fn test_straight_flush_beats_four_of_a_kind() {
    test(&["4P 5C 5P 5D 5T", "7P 8P 9P 6P 10P"], &["7P 8P 9P 6P 10P"])
}

#[test]
#[ignore]
fn test_straight_flush_ranks() {
    // both hands have straight flush, tie goes to highest-ranked card
    test(&["4C 6C 7C 8C 5C", "5P 7P 8P 9P 6P"], &["5P 7P 8P 9P 6P"])
}
