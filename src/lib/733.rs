// Time taken: 20:56, 21:09, 21:12 -> Acc

struct Solution {}

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let sr = sr as usize;
        let sc = sc as usize;
        let adj = vec![(0, -1), (-1, 0), (0, 1), (1, 0)];
        let mut image = image;
        let orig_color = image[sr][sc];

        if color == orig_color {
            return image;
        }

        Self::dfs(&mut image, sr, sc, color, orig_color, &adj);

        return image;
    }

    fn dfs(
        mut image: &mut Vec<Vec<i32>>,
        sr: usize,
        sc: usize,
        color: i32,
        orig_color: i32,
        adj: &Vec<(i32, i32)>,
    ) {
        if image[sr][sc] != orig_color || image[sr][sc] == color {
            return;
        }
        image[sr][sc] = color;

        for (ii, jj) in adj {
            if sr as i32 + ii < 0 || sc as i32 + jj < 0 {
                continue;
            }
            let posx = (sr as i32 + ii) as usize;
            let posy = (sc as i32 + jj) as usize;
            if let Some(row) = image.get(posx) {
                if let Some(&val) = row.get(posy) {
                    if val == orig_color {
                        Self::dfs(&mut image, posx, posy, color, orig_color, &adj);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
