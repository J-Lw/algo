// Leetcode p280 - Wiggle Sort.
// https://leetcode.com/problems/wiggle-sort/description/

fn wiggle_sort(numbers: &mut Vec<i32>) {
    numbers.sort();
    let mut product = Vec::new();

    for (index, number) in numbers.iter().enumerate() { 
        product.insert(number);
        product.push(sorted_numbers[index + 2]);
        product.push(sorted_numbers[index + 1]);
    }

    product
}   

#[cfg(test)]
mod tests {
    use super::wiggle_sort;

    #[test]
    fn standard_case() {
        let input: Vec<i32> = vec![3, 5, 2, 1, 6, 4];
        let target_product = vec![3, 5, 1, 6, 2, 4];
        assert_eq!(wiggle_sort(&input), target_product);
    }
}
