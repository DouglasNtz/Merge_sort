#[test]
fn test_my_merge_sort() {

    let mut v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    super::algorithms::my_merge_sort(&mut v);
    assert_eq!(v, [0,0,0,1,2,3,3,5,5,5,6,9,9,10,10])
}
#[test]
fn test_my_merge_sort_proof_stability() {

    let mut v = vec![(10,1),(5,1),(2,1),(9,2),(0,2),(3,2),(3,1),(0,3),(9,1),(1,1),(5,3),(0,1),(10,2),(5,2),(6,1)];
    super::proof_algorithms::my_merge_sort_proof_stability(&mut v);
    assert_eq!(v, [(0,2),(0,3),(0,1),(1,1),(2,1),(3,2),(3,1),(5,1),(5,3),(5,2),(6,1),(9,2),(9,1),(10,1),(10,2)])
}

#[test]
fn test_my_merge_sort_optmized() {

    let mut v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    super::algorithms::my_merge_sort_optmized(&mut v);
    assert_eq!(v, [0,0,0,1,2,3,3,5,5,5,6,9,9,10,10])
}
#[test]
fn test_merge_sort_optmized_proof_stability() {

    let mut v = vec![(10,1),(5,1),(2,1),(9,2),(0,2),(3,2),(3,1),(0,3),(9,1),(1,1),(5,3),(0,1),(10,2),(5,2),(6,1)];
    super::proof_algorithms::my_merge_sort_optmized_proof_stability(&mut v);
    assert_eq!(v, [(0,2),(0,3),(0,1),(1,1),(2,1),(3,2),(3,1),(5,1),(5,3),(5,2),(6,1),(9,2),(9,1),(10,1),(10,2)])
}