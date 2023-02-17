use std::collections::HashSet;
use std::hash::Hash;

fn unique<T>(list: Vec<T>) -> Vec<T> 
where T: Ord + Hash + Copy {
    if list.len() < 2 {
        return list;
    }

    let mut unique_list: Vec<T> = Vec::new();
    let mut used_elements: HashSet<T> = HashSet::new();

    for x in list {
        if !used_elements.contains(&x) {
            used_elements.insert(x);
            unique_list.push(x);
        }
    }

    unique_list
}

fn unique_short<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    list.sort();
    list.dedup();
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_duplicates_on_input() {
        let list = vec![1, 4, 5];
        assert_eq!(unique(list), vec![1, 4, 5]);
    }

    #[test]
    fn duplicates_on_input() {
        let list = vec![1, 1, 3];
        assert_eq!(unique(list), vec![1, 3]);
    }
}