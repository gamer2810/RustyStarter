use crate::Sort;

#[derive(Default)]
pub struct InsertionSort {}

impl Sort for InsertionSort {
    fn sort(&self, input: Vec<i32>) -> Vec<i32> {
        let len: usize = input.len();
        let mut result = Vec::with_capacity(len);

        for i in 0 .. len {
            result.push(input[i])
        }

        for num in 1 .. len -1  {
            let mut lookup = num;

            while lookup > 0 && result[lookup - 1] > result[lookup] {
                let temp = result[lookup-1];
                result[lookup-1] = result [lookup];
                result[lookup] = temp;
                lookup -= 1;
            }
            // result[lookup + 1] = key;
        }

        return result;
    }
}