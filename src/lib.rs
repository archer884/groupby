use std::collections::HashMap;
use std::hash::Hash;

pub trait GroupBy: IntoIterator + Sized {
    fn group_by<F, K>(self, mut derive_key: F) -> HashMap<K, Vec<Self::Item>>
    where
        F: FnMut(&Self::Item) -> K,
        K: Eq + Hash,
    {
        let mut map = HashMap::new();
        self.into_iter().for_each(|item| {
            map.entry(derive_key(&item))
                .or_insert_with(Vec::new)
                .push(item)
        });
        map
    }
}

impl<T: IntoIterator + Sized> GroupBy for T {}

#[cfg(test)]
mod tests {
    use GroupBy;

    #[test]
    fn it_works() {
        let input = "abbccc";
        let grouped = input.chars().group_by(|&c| c);

        assert_eq!(&['a'], &*grouped[&'a']);
        assert_eq!(&['b', 'b'], &*grouped[&'b']);
        assert_eq!(&['c', 'c', 'c'], &*grouped[&'c']);
    }
}
