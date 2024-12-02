use day_2::*;

#[test]
fn test_list_similarity_score() {
    let score = list_similarity_score(
        vec![3, 4, 2, 1, 3, 3],
        vec![4, 3, 5, 3, 9, 3]
    );

    assert_eq!(score, 31);
}
