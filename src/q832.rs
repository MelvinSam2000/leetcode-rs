/*
    832 - Flipping Image
    Time: O(n^2)
    Space: O(1)
*/
pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = image.len();
    for row in image.iter_mut() {
        for j in 0..n / 2 {
            row.swap(j, n - j - 1);
        }
        for elem in row.iter_mut() {
            *elem = 1 - *elem;
        }
    }
    image
}
