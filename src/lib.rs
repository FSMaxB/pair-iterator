pub struct Pairs<IteratorType: Iterator<Item = ItemType>, ItemType> {
    iterator: IteratorType,
}

impl<IteratorType: Iterator<Item = ItemType>, ItemType> Pairs<IteratorType, ItemType> {
    fn new(iterator: IteratorType) -> Pairs<IteratorType, ItemType> {
        Pairs {
            iterator,
        }
    }
}

impl<IteratorType: Iterator<Item = ItemType>, ItemType> Iterator for Pairs<IteratorType, ItemType> {
    type Item = (ItemType, ItemType);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub trait PairIterator {
    type Item;
    type Iterator: Iterator<Item = Self::Item>;

    fn pairs(self) -> Pairs<Self::Iterator, Self::Item>;
}

impl<IteratorType: Iterator<Item = ItemType>, ItemType> PairIterator for IteratorType {
    type Item = ItemType;
    type Iterator = Self;

    fn pairs(self) -> Pairs<Self::Iterator, Self::Item> {
        Pairs::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::PairIterator;

    #[test]
    fn should_provide_nothing_for_only_one_input() {
        let array = [1];
        let mut iterator = array.iter().pairs();

        assert_eq!(None, iterator.next());
    }
}