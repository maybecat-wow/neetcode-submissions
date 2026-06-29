impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<usize>> = HashMap::new();
        let mut count: Vec<[u8; 26]> = vec![[0; 26]; strs.len()];
        for (idx, str) in strs.iter().enumerate() {
            for letter in str.chars() {
                *count
                    .get_mut(idx)
                    .unwrap()
                    .get_mut((letter as usize)-('a' as usize))
                    .unwrap() += 1;
            }
        }
        for (idx, item) in count.iter().enumerate() {
            if let Some(x) = map.get_mut(item) {
                x.push(idx);
            } else {
                map.insert(*item, vec![idx]);
            }
        }
        let mut ans: Vec<Vec<String>> = Vec::new();
        for (_, item) in map.iter() {
            let mut temp = Vec::new();
            for idx in item.iter() {
                temp.push(strs.get(*idx).unwrap().clone());
            }
            ans.push(temp);
        }
        ans
    }
}
