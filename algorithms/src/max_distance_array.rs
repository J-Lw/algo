// Leetcode p624 - Maximum Distance in Arrays.
// https://leetcode.com/problems/maximum-distance-in-arrays/description/

fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    
}

#[cfg(test)]
mod tests {
    use super::max_distance;

    #[test]
    fn standard_case() {
        let input: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 5],
            vec![1, 2, 3],
        ];
        let target_product: i32 = 4;
        assert_eq!(max_distance(input), target_product);
    }
}
