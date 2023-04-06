fn insertion_sort(arr: &mut [i32]) -> &[i32] {
    let mut j: usize;

    for i in 1..arr.len() {
        j = i;

        while j > 0 && arr[j - 1] > arr[j] {
            (arr[j], arr[j - 1]) = (arr[j - 1], arr[j]);

            j = j - 1;
        }
    }

    arr
}

#[test]
fn test_insertion_sort() {
    let mut arr = [5, 2, 4, 6, 1, 3];
    insertion_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6]);

    let mut arr = [1, 2, 3, 4, 5, 6];
    insertion_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6]);

    let mut arr = [6, 5, 4, 3, 2, 1];
    insertion_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
}
