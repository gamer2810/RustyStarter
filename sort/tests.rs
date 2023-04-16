macro_rules! sort_tests {
    ($($name:ident: $type:ty,)*) => {
    $(
        #[test]
        fn $name() {
            let c = <$type>::default();
            let input = vec![3,2,5];
            let expected = vec![2,3,5];
            assert_eq!(expected, <$type>::sort(&c, input));
        }
    )*
    }
}

#[cfg(test)]
mod tests {
    use crate::insertion::InsertionSort;
    use crate::Sort;

    sort_tests! {
        insertion: InsertionSort,
    }
}