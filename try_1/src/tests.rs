use crate::Solution;

#[test]
#[rustfmt::skip]
fn josephs_ring() {
    let result = Solution::josephs_ring(10, 4);
    assert_eq!(
        result,
        vec![
            false, true,  true,  false, false,
            true,  false, false, true,  false,
            true,  false, true,  true,  false,
            false, true,  true,  true,  false,
        ]
    );
}

#[test]
fn is_palindrome() {
    assert_eq!(Solution::is_palindrome("madam, i am a man."), false);
    assert_eq!(Solution::is_palindrome("madam, I'm adam."), true);
    assert_eq!(Solution::is_palindrome("0P"), false); // leetcode
}

#[test]
fn tower_of_hanoi() {
    let src = &mut vec![];
    let tmp = &mut vec![];
    let dist = &mut vec![];
    for i in (0..10).rev() {
        src.push(i);
    }
    let result = src.clone();
    println!("{:?}", result);
    Solution::tower_of_hanoi(src, tmp, dist);
    assert_eq!(*src, vec![]);
    assert_eq!(*tmp, vec![]);
    assert_eq!(*dist, result);
}

#[test]
fn permutation() {
    let vec = vec![1, 2, 3];
    // Next line let `cnt = vec.len()` to do a full permutation.
    let result = Solution::permutation(&vec, vec.len());
    println!("{:?}", result);
    assert_eq!(result.len(), 6);
    assert!(result.contains(&vec![1, 2, 3]));
    assert!(result.contains(&vec![1, 3, 2]));
    assert!(result.contains(&vec![2, 1, 3]));
    assert!(result.contains(&vec![2, 3, 1]));
    assert!(result.contains(&vec![3, 1, 2]));
    assert!(result.contains(&vec![3, 2, 1]));
}
