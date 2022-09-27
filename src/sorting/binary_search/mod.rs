use std::fmt::Debug;

pub fn binary_search<R: AsRef<[T]>, T: Ord + Debug>(a: &T, arr: &R) -> Option<usize> {
    let arr = arr.as_ref();
    let mut temp_arr = arr;
    let mut adding_index = 0;
    let mut is_asc = true;
    if arr.len() > 1 && arr.first() > arr.last() {
        is_asc = false;
    }

    loop {
        if temp_arr.is_empty() {
            return None;
        }

        let mid_index = temp_arr.len() / 2;
        let mid_item = match temp_arr.get(mid_index) {
            None => {
                return None;
            }
            Some(item) => item,
        };

        if is_asc {
            match mid_item.cmp(a) {
                std::cmp::Ordering::Less => {
                    adding_index += mid_index + 1;
                    temp_arr = &temp_arr[mid_index + 1..];
                }
                std::cmp::Ordering::Equal => {
                    return Some(adding_index + mid_index);
                }
                std::cmp::Ordering::Greater => {
                    temp_arr = &temp_arr[..mid_index];
                }
            }
        } else {
            match mid_item.cmp(a) {
                std::cmp::Ordering::Greater => {
                    adding_index += mid_index + 1;
                    temp_arr = &temp_arr[mid_index + 1..];
                }
                std::cmp::Ordering::Equal => {
                    return Some(adding_index + mid_index);
                }
                std::cmp::Ordering::Less => {
                    temp_arr = &temp_arr[..mid_index];
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = binary_search(&"a", &vec![]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = binary_search(&"a", &vec!["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings_asc() {
        let index = binary_search(&"a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));

        let index = binary_search(&"google", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(4));
    }

    #[test]
    fn search_strings_desc() {
        let index = binary_search(&"a", &vec!["zoo", "google", "d", "c", "b", "a"]);
        assert_eq!(index, Some(5));

        let index = binary_search(&"zoo", &vec!["zoo", "google", "d", "c", "b", "a"]);
        assert_eq!(index, Some(0));

        let index = binary_search(&"google", &vec!["zoo", "google", "d", "c", "b", "a"]);
        assert_eq!(index, Some(1));
    }

    #[test]
    fn search_ints_asc() {
        let index = binary_search(&4, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = binary_search(&3, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = binary_search(&2, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = binary_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints_desc() {
        let index = binary_search(&4, &vec![4, 3, 2, 1]);
        assert_eq!(index, Some(0));

        let index = binary_search(&3, &vec![4, 3, 2, 1]);
        assert_eq!(index, Some(1));

        let index = binary_search(&2, &vec![4, 3, 2, 1]);
        assert_eq!(index, Some(2));

        let index = binary_search(&1, &vec![4, 3, 2, 1]);
        assert_eq!(index, Some(3));
    }

    #[test]
    fn not_found() {
        let index = binary_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);

        let index = binary_search(&5, &vec![4, 3, 2, 1]);
        assert_eq!(index, None);
    }
}
