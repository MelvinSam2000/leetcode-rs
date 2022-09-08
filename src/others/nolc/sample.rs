pub fn minimal_heaviest_set_a(
    mut arr: Vec<i32>,
) -> Vec<i32> {
    arr.sort();
    let s: i32 = arr.iter().sum();
    let mut a_sum = 0;
    let mut b_sum = s;
    let mut index = 0;
    for i in (0..arr.len()).rev() {
        if a_sum > b_sum {
            break;
        }
        a_sum += arr[i];
        b_sum -= arr[i];
        index = i;
    }
    arr[index..].into()
}

#[test]
fn t1() {
    let tcases = [
        (vec![3, 7, 5, 6, 2], vec![6, 7]),
        (vec![5, 3, 2, 4, 1, 2], vec![4, 5]),
        (vec![4, 2, 5, 1, 6], vec![5, 6]),
    ];
    for (param, res) in tcases {
        assert_eq!(minimal_heaviest_set_a(param), res);
    }
}
