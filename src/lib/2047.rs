// Time taken: 19:40, 19:55 -> Wrong, 19:57 -> Acc
struct Solution {}

impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let mut result = 0;
        let mut idx = 0;
        let sentence = sentence.chars().collect::<Vec<char>>();
        let mut word = Vec::new();

        while idx < sentence.len() {
            if sentence[idx] != ' ' {
                word.push(sentence[idx]);
            } else {
                if !word.is_empty() {
                    if Self::is_valid(&word) {
                        result += 1;
                    }
                    word = Vec::new();
                }
            }
            idx += 1;
        }

        if !word.is_empty() {
            if Self::is_valid(&word) {
                result += 1;
            }
        }

        return result;
    }

    pub fn is_valid(arr: &Vec<char>) -> bool {
        let mut idx = 0;
        let mut hyphen_len = (0, 0);
        let mut punctuation_len = (0, 0);

        while idx < arr.len() {
            if arr[idx].is_ascii_digit() {
                return false;
            }

            if arr[idx] == '-' {
                hyphen_len.0 += 1;
                hyphen_len.1 = idx;
                if hyphen_len.0 == 2 {
                    return false;
                }
            } else {
                if arr[idx] == '!' || arr[idx] == '.' || arr[idx] == ',' {
                    punctuation_len.0 += 1;
                    punctuation_len.1 = idx;
                    if punctuation_len.0 == 2 {
                        return false;
                    }
                }
            }

            idx += 1;
        }

        if hyphen_len.0 == 1 {
            if hyphen_len.1 == 0 || hyphen_len.1 == arr.len() - 1 {
                return false;
            }
            if !arr[hyphen_len.1 - 1].is_ascii_alphabetic()
                || !arr[hyphen_len.1 + 1].is_ascii_alphabetic()
            {
                return false;
            }
        }

        if punctuation_len.0 == 1 {
            if punctuation_len.1 != arr.len() - 1 {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_valid_words("cat and  dog".to_string()), 3);
        assert_eq!(
            Solution::count_valid_words("!this  1-s b8d!".to_string()),
            0
        );
        assert_eq!(
            Solution::count_valid_words("alice and  bob are playing stone-game10".to_string()),
            5
        );
        assert_eq!(
            Solution::count_valid_words(
            "qte1i   1-,, yv03a  r12r2stw 4 d,tnirlsj pb !16- 9 b  dnlgrig 8   n!88qyfjly   0g f5hgfg0u9lux7 - 6ega 0p36 pnw  ae  0m  -v  q3zdw09b9qju q0! s-  jk 04 e1ik  2 3  k a1qe.ac,-w j,keef76xz8  -!zhc s b u -z. ,,b -rei 83ooj 899 af w1irv u o3jk21 71i60pq3,.rzbhc.-  t9 xlk5g  ovn 8f9  ztw 7siy p-yl856r, ma39xtl!t-o c 2x 2 drj!ms0w ysy  u0tcw8u.im c 0ke.5sk  dn8.mh qi   8xmt -bxmr  z 1r 5 umyk 8rbe!dif kmes n rp icnb s 0yc1e 8  e1 !f  .u lh  n a -iinnm!a08dfgq ,lux,j 8fyqt hcbajnb4swuxtqm4j1  ic04 o,i4lka id 0srlb  y 2k  1g  3m nptj   53rh, zim7mkd2hqf64 chotiijcemj!m dif7iiq m2e ve!9!r1 jw okyahf! r6kskaodd h eug.yc,3j ilkd 9vlpipfc  g5y   7u 5pt531!4s 4  si !lg x50-   kc51ca34s pl 9w,mgj3  5fy,.3d shi ct a k2nx8l xum9sgyp6r   rj! 8  m!1k gm  typy . oee08!!j0,2iwq  9 ywd w rhpoc s6118y c5.qw4d  tlrjs.!9 mpioexe. xmicv  ,z g2 p6 bhtm!  ,w 7 ".to_string()
        ),
            50
        );
    }
}
