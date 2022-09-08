/*
    49 - Group Anagram
    Time: O(n*m*logm)
    Space: O(n)
    Note: n is the length of strs, m is the length of each string in strs
*/
pub fn group_anagrams(
    words: Vec<String>,
) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let sorted_words = words
        .iter()
        .map(|word| {
            let mut chars =
                word.chars().collect::<Vec<char>>();
            chars.sort();
            chars
        })
        .map(String::from_iter)
        .collect::<Vec<String>>();
    let mut grouped = HashMap::<String, Vec<String>>::new();
    for (word, sorted_word) in
        words.into_iter().zip(sorted_words)
    {
        if let Some(group) = grouped.get_mut(&sorted_word) {
            group.push(word);
        } else {
            grouped.insert(sorted_word, vec![word]);
        }
    }
    grouped.into_values().collect()
}
