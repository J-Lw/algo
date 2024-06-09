// Leetcode p624 - Maximum Distance in Arrays.
// https://leetcode.com/problems/maximum-distance-in-arrays/description/

fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    let mut min_n = arrays[0][0];
    let mut max_n = arrays[0][0];

    for array in &arrays {
        let first_element = array[0];
        let last_element = array[array.len() - 1];

        if first_element < min_n {
            min_n = first_element;
        }

        if last_element > max_n {
            max_n = last_element;
        }
    }

    max_n - min_n
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
