use hamming;

fn process_distance_case(strand_pair: [&str; 2], expected_distance: Option<usize>) {
    assert_eq!(
        hamming::hamming_distance(strand_pair[0], strand_pair[1]),
        expected_distance
    );
}

#[test]
fn test_empty_strands() {
    process_distance_case(["", ""], Some(0));
}

#[test]
fn test_no_difference_between_identical_strands() {
    process_distance_case(["GGACTGA", "GGACTGA"], Some(0));
}

#[test]
fn test_complete_hamming_distance_in_small_strand() {
    process_distance_case(["ACT", "GGA"], Some(3));
}

#[test]
fn test_small_hamming_distance_in_the_middle_somewhere() {
    process_distance_case(["GGACG", "GGTCG"], Some(1));
}

#[test]
fn test_larger_distance() {
    process_distance_case(["ACCAGGG", "ACTATGG"], Some(2));
}

#[test]
fn test_first_string_is_longer() {
    process_distance_case(["AAA", "AA"], None);
}

#[test]
fn test_second_string_is_longer() {
    process_distance_case(["A", "AA"], None);
}

#[test]
/// non-unique character in first strand
fn test_nonunique_character_in_first_strand() {
    process_distance_case(["AAG", "AAA"], Some(1));
}

#[test]
/// identical strands
fn test_identical_strands() {
    process_distance_case(["A", "A"], Some(0));
}

#[test]
/// complete distance in small strands
fn test_complete_distance_in_small_strands() {
    process_distance_case(["AG", "CT"], Some(2));
}

#[test]
/// disallow first strand longer
fn test_disallow_first_strand_longer() {
    process_distance_case(["AATG", "AAA"], None);
}

#[test]
/// large distance
fn test_large_distance() {
    process_distance_case(["GATACA", "GCATAA"], Some(4));
}

#[test]
/// long identical strands
fn test_long_identical_strands() {
    process_distance_case(["GGACTGA", "GGACTGA"], Some(0));
}

#[test]
/// complete distance in single nucleotide strands
fn test_complete_distance_in_single_nucleotide_strands() {
    process_distance_case(["A", "G"], Some(1));
}

#[test]
/// small distance
fn test_small_distance() {
    process_distance_case(["GGACG", "GGTCG"], Some(1));
}

#[test]
/// non-unique character in second strand
fn test_nonunique_character_in_second_strand() {
    process_distance_case(["AAA", "AAG"], Some(1));
}

#[test]
/// small distance in long strands
fn test_small_distance_in_long_strands() {
    process_distance_case(["ACCAGGG", "ACTATGG"], Some(2));
}

#[test]
/// disallow second strand longer
fn test_disallow_second_strand_longer() {
    process_distance_case(["ATA", "AGTG"], None);
}

#[test]
/// small distance in small strands
fn test_small_distance_in_small_strands() {
    process_distance_case(["AT", "CT"], Some(1));
}

#[test]
/// large distance in off-by-one strand
fn test_large_distance_in_offbyone_strand() {
    process_distance_case(["GGACGGATTCTG", "AGGACGGATTCT"], Some(9));
}

#[test]
/// same nucleotides in different positions
fn test_same_nucleotides_in_different_positions() {
    process_distance_case(["TAG", "GAT"], Some(2));
}
