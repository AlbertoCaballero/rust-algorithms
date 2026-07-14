use std::collections::HashSet;

impl Solution {
    pub fn word_break(word: String, word_dict: Vec<String>) -> bool {
        let dict_set: HashSet<String> = word_dict.into_iter().collect();
        let chars: Vec<char> = word.chars().collect();
        let n = chars.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;

        for i in 1..=n {
            for j in 0..i {
                if dp[j] {
                    let substring: String = chars[j..i].iter().collect();
                    if dict_set.contains(substring.as_str()) {
                        dp[i] = true;
                        break;
                    }
                }
            }
        }

        dp[n]
    }
}
