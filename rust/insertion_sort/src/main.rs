fn main() {
    println!("Insertion sort");
}

pub fn insertion_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    for i in 1..a.len() {
        for j in (1..=i).rev() {
            if a[j] < a[j - 1] {
                (a[j], a[j - 1]) = (a[j - 1], a[j])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut a: Vec<i32> = vec![];
        insertion_sort(&mut a);
        assert_eq!(a, vec![]);
    }

    #[test]
    fn one() {
        let mut a = vec![1];
        insertion_sort(&mut a);
        assert_eq!(a, vec![1]);
    }

    #[test]
    fn two() {
        let mut a = vec![11, 1];
        insertion_sort(&mut a);
        assert_eq!(a, vec![1, 11]);
    }

    #[test]
    fn repeating_terms() {
        let mut a = vec![11, 1, 51, 1, 5, 3];
        insertion_sort(&mut a);
        assert_eq!(a, vec![1, 1, 3, 5, 11, 51]);
    }

    #[test]
    fn sorted() {
        let mut a = (0..50).collect::<Vec<_>>();
        insertion_sort(&mut a);
        assert_eq!(a, (0..50).collect::<Vec<_>>());
    }

    #[test]
    fn reversed_sorted() {
        let mut a = (0..50).rev().collect::<Vec<_>>();
        insertion_sort(&mut a);
        assert_eq!(a, (0..50).collect::<Vec<_>>());
    }

    #[test]
    fn negative_number() {
        let mut a = vec![1, 1, -5, 7];
        insertion_sort(&mut a);
        assert_eq!(a, vec![-5, 1, 1, 7]);
    }

    #[test]
    fn float_number() {
        let mut a: Vec<f32> = vec![11.0, -4.0, 20.0, 15.9, 13.5, -20.0];
        insertion_sort(&mut a);
        assert_eq!(a, vec![-20.0, -4.0, 11.0, 13.5, 15.9, 20.0]);
    }
}
