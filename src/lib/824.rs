// Time taken: 02:19, 02:26, 02:29 -> Wrong, 02:31 -> Acc
struct Solution {}

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let mut result = String::new();
        let sentence = sentence.split_ascii_whitespace().into_iter();
        let mut count = 1;

        for word in sentence {
            let mut word_vec = word.chars().collect::<Vec<char>>();
            if word_vec[0].to_ascii_lowercase() == 'a'
                || word_vec[0].to_ascii_lowercase() == 'e'
                || word_vec[0].to_ascii_lowercase() == 'i'
                || word_vec[0].to_ascii_lowercase() == 'o'
                || word_vec[0].to_ascii_lowercase() == 'u'
            {
                result = result + word + "ma" + &vec!["a"; count].concat() + " ";
            } else {
                let letter = word_vec.remove(0);
                word_vec.push(letter);
                result = result
                    + &word_vec.iter().collect::<String>()
                    + "ma"
                    + &vec!["a"; count].concat()
                    + " ";
            }
            count += 1;
        }

        return result.trim_end().to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::to_goat_latin("I speak Goat Latin".to_string()),
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
        );
        assert_eq!(
            Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_string()),
            "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"
        );
        assert_eq!(
            Solution::to_goat_latin("Each word consists of lowercase and uppercase letters only".to_string()),
            "Eachmaa ordwmaaa onsistscmaaaa ofmaaaaa owercaselmaaaaaa andmaaaaaaa uppercasemaaaaaaaa etterslmaaaaaaaaa onlymaaaaaaaaaa"
        );
    }
}
