use std::collections::BTreeSet;

/// An iterator which generates combinations over a set of elements.
///
/// # Examples
///
/// ```
/// use combinatorial::Combinations;
///
/// let mut abc_tuples = Combinations::of_size(vec!['a', 'b', 'c'], 2);
/// assert_eq!(abc_tuples.next(), Some(vec!['a', 'b']));
/// assert_eq!(abc_tuples.next(), Some(vec!['a', 'c']));
/// assert_eq!(abc_tuples.next(), Some(vec!['b', 'c']));
/// assert_eq!(abc_tuples.next(), None);
///
/// let ones_and_zeros: Vec<Vec<usize>> = Combinations::all(0..2).collect();
/// assert_eq!(ones_and_zeros, vec![Vec::new(), vec![0], vec![1], vec![0, 1]]);
/// ```
pub struct Combinations<T> {
    elements: Vec<T>,
    positions: Vec<usize>,
    all_sizes: bool,
    done: bool,
}

/// Converts an iterable input into a sorted vector containing one of every unique item from the
/// original iterable.
fn iterable_to_sorted_set<T: Ord + Clone>(elements: impl IntoIterator<Item = T>) -> Vec<T> {
    elements
        .into_iter()
        .collect::<BTreeSet<T>>()
        .into_iter()
        .collect::<Vec<T>>()
}

impl<T: Ord + Clone> Combinations<T> {
    /// Creates a new `Combinations` iterator which will yield all combinations of the elements in
    /// the given iterable.
    ///
    /// # Examples
    ///
    /// ```
    /// use combinatorial::Combinations;
    ///
    /// let mut combos = Combinations::all(vec!["hello", "world"]).map(|str_vec| str_vec.join(" "));
    /// assert_eq!(combos.next(), Some(String::from("")));
    /// assert_eq!(combos.next(), Some(String::from("hello")));
    /// assert_eq!(combos.next(), Some(String::from("world")));
    /// assert_eq!(combos.next(), Some(String::from("hello world")));
    /// assert_eq!(combos.next(), None);
    ///
    /// let mut combos = Combinations::all(1..1);
    /// assert_eq!(combos.next(), Some(Vec::new()));
    /// assert_eq!(combos.next(), None);
    /// ```
    pub fn all(elements: impl IntoIterator<Item = T>) -> Self {
        Combinations {
            elements: iterable_to_sorted_set(elements),
            positions: Vec::new(),
            all_sizes: true,
            done: false,
        }
    }

    /// Creates a new `Combinations` iterator which will yield all combinations with the specified
    /// size from the elements in the given iterable.
    ///
    /// # Examples
    ///
    /// ```
    /// use combinatorial::Combinations;
    ///
    /// let mut combos = Combinations::of_size(1..4, 2);
    /// assert_eq!(combos.next(), Some(vec![1, 2]));
    /// assert_eq!(combos.next(), Some(vec![1, 3]));
    /// assert_eq!(combos.next(), Some(vec![2, 3]));
    /// assert_eq!(combos.next(), None);
    ///
    /// let mut combos = Combinations::of_size('a'..'z', 0);
    /// assert_eq!(combos.next(), Some(Vec::new()));
    /// assert_eq!(combos.next(), None);
    ///
    /// let mut combos = Combinations::of_size(vec!["foo", "bar", "baz"], 4);
    /// assert_eq!(combos.next(), None);
    /// ```
    pub fn of_size(elements: impl IntoIterator<Item = T>, size: usize) -> Self {
        Combinations {
            elements: iterable_to_sorted_set(elements),
            positions: (0..size).collect(),
            all_sizes: false,
            done: false,
        }
    }

    /// Adds another position indicator to the internal positions list and resets them to point to
    /// the first `n` indices in order.
    fn move_to_next_set_size(&mut self) -> bool {
        if self.positions.len() >= self.elements.len() {
            return false;
        }
        self.positions
            .iter_mut()
            .enumerate()
            .for_each(|(index, pos)| *pos = index);
        self.positions.push(self.positions.len());
        true
    }

    /// Increments the internal positions to correspond to the indices of the next combination of
    /// the same size.  If the positions are successfully incremented at the current combination
    /// set size, then returns `true`.  Otherwise, returns `false`.
    fn move_to_next_position(&mut self) -> bool {
        if self.elements.len() == 0 {
            return false;
        }
        let length = self.positions.len();
        for index in (0..self.positions.len()).rev() {
            let cur_position = *self.positions.get(index).unwrap();
            if cur_position >= self.elements.len() - 1 {
                continue;
            }
            if index == length - 1 || cur_position < self.positions.get(index + 1).unwrap() - 1 {
                let mut next_position = cur_position + 1;
                *self.positions.get_mut(index).unwrap() = next_position;
                for i in index + 1..length {
                    next_position += 1;
                    *self.positions.get_mut(i).unwrap() = next_position;
                }
                return true;
            }
        }
        false
    }

    /// Returns the current combination, if one exists and is valid.
    fn get_current_combination(&mut self) -> Option<Vec<T>> {
        if self.done || self.positions.len() > self.elements.len() {
            return None;
        }
        Some(
            self.positions
                .iter()
                .map(|p| self.elements.get(*p).unwrap().clone())
                .collect::<Vec<T>>(),
        )
    }
}

impl<T: Ord + Clone> Iterator for Combinations<T> {
    type Item = Vec<T>;

    /// Returns the next combination and advances the internal iterator.
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let combo = self.get_current_combination();
        if self.move_to_next_position() == false {
            if self.all_sizes == false || self.move_to_next_set_size() == false {
                self.done = true;
            }
        }
        combo
    }
}

/// An iterator which generates combinations over a set of elements, with replacement.
///
/// # Examples
///
/// ```
/// use combinatorial::CombinationsWithReplacement;
///
/// let mut abc_tuples = CombinationsWithReplacement::of_size(vec!['a', 'b', 'c'], 2);
/// assert_eq!(abc_tuples.next(), Some(vec!['a', 'a']));
/// assert_eq!(abc_tuples.next(), Some(vec!['a', 'b']));
/// assert_eq!(abc_tuples.next(), Some(vec!['a', 'c']));
/// assert_eq!(abc_tuples.next(), Some(vec!['b', 'b']));
/// assert_eq!(abc_tuples.next(), Some(vec!['b', 'c']));
/// assert_eq!(abc_tuples.next(), Some(vec!['c', 'c']));
/// assert_eq!(abc_tuples.next(), None);
///
/// let mut ones_and_zeros = CombinationsWithReplacement::all(0..2);
/// assert_eq!(ones_and_zeros.next(), Some(Vec::new()));
/// assert_eq!(ones_and_zeros.next(), Some(vec![0]));
/// assert_eq!(ones_and_zeros.next(), Some(vec![1]));
/// assert_eq!(ones_and_zeros.next(), Some(vec![0, 0]));
/// assert_eq!(ones_and_zeros.next(), Some(vec![0, 1]));
/// assert_eq!(ones_and_zeros.next(), Some(vec![1, 1]));
/// assert_eq!(ones_and_zeros.next(), None);
/// ```
pub struct CombinationsWithReplacement<T> {
    elements: Vec<T>,
    positions: Vec<usize>,
    all_sizes: bool,
    done: bool,
}

impl<T: Ord + Clone> CombinationsWithReplacement<T> {
    /// Creates a new `CombinationsWithReplacement` iterator which will yield all combinations with
    /// replacement of the elements in the given iterable.
    ///
    /// # Examples
    ///
    /// ```
    /// use combinatorial::CombinationsWithReplacement;
    ///
    /// let mut combos = CombinationsWithReplacement::all(vec!["hello", "world"]).map(|str_vec| str_vec.join(" "));
    /// assert_eq!(combos.next(), Some(String::from("")));
    /// assert_eq!(combos.next(), Some(String::from("hello")));
    /// assert_eq!(combos.next(), Some(String::from("world")));
    /// assert_eq!(combos.next(), Some(String::from("hello hello")));
    /// assert_eq!(combos.next(), Some(String::from("hello world")));
    /// assert_eq!(combos.next(), Some(String::from("world world")));
    /// assert_eq!(combos.next(), None);
    ///
    /// let mut combos = CombinationsWithReplacement::all(1..1);
    /// assert_eq!(combos.next(), Some(Vec::new()));
    /// assert_eq!(combos.next(), None);
    /// ```
    pub fn all(elements: impl IntoIterator<Item = T>) -> Self {
        CombinationsWithReplacement {
            elements: iterable_to_sorted_set(elements),
            positions: Vec::new(),
            all_sizes: true,
            done: false,
        }
    }

    /// Creates a new `CombinationsWithReplacement` iterator which will yield all combinations with
    /// replacement of the specified size from the elements in the given iterable.
    ///
    /// # Examples
    ///
    /// ```
    /// use combinatorial::CombinationsWithReplacement;
    ///
    /// let mut combos = CombinationsWithReplacement::of_size(1..4, 2);
    /// assert_eq!(combos.next(), Some(vec![1, 1]));
    /// assert_eq!(combos.next(), Some(vec![1, 2]));
    /// assert_eq!(combos.next(), Some(vec![1, 3]));
    /// assert_eq!(combos.next(), Some(vec![2, 2]));
    /// assert_eq!(combos.next(), Some(vec![2, 3]));
    /// assert_eq!(combos.next(), Some(vec![3, 3]));
    /// assert_eq!(combos.next(), None);
    ///
    /// let mut combos = CombinationsWithReplacement::of_size('a'..'z', 0);
    /// assert_eq!(combos.next(), Some(Vec::new()));
    /// assert_eq!(combos.next(), None);
    ///
    /// let mut combos = CombinationsWithReplacement::of_size(vec!["foo", "bar", "baz"], 4);
    /// assert_eq!(combos.next(), None);
    /// ```
    pub fn of_size(elements: impl IntoIterator<Item = T>, size: usize) -> Self {
        CombinationsWithReplacement {
            elements: iterable_to_sorted_set(elements),
            positions: vec![0; size],
            all_sizes: false,
            done: false,
        }
    }

    /// Adds another position indicator to the internal positions list and resets them to point to
    /// the first index of the elements.
    fn move_to_next_set_size(&mut self) -> bool {
        if self.positions.len() >= self.elements.len() {
            return false;
        }
        self.positions.iter_mut().for_each(|pos| *pos = 0);
        self.positions.push(0);
        true
    }

    /// Increments the internal positions to correspond to the indices of the next combination of
    /// the same size.  If the positions are successfully incremented at the current combination
    /// set size, then returns `true`.  Otherwise, returns `false`.
    fn move_to_next_position(&mut self) -> bool {
        if self.elements.len() == 0 {
            return false;
        }
        let length = self.positions.len();
        for index in (0..self.positions.len()).rev() {
            let cur_position = *self.positions.get(index).unwrap();
            if cur_position >= self.elements.len() - 1 {
                continue;
            }
            let next_position = cur_position + 1;
            for i in index..length {
                *self.positions.get_mut(i).unwrap() = next_position;
            }
            return true;
        }
        false
    }

    /// Returns the current combination, if one exists and is valid.
    fn get_current_combination(&mut self) -> Option<Vec<T>> {
        if self.done || self.positions.len() > self.elements.len() {
            return None;
        }
        Some(
            self.positions
                .iter()
                .map(|p| self.elements.get(*p).unwrap().clone())
                .collect::<Vec<T>>(),
        )
    }
}

impl<T: Ord + Clone> Iterator for CombinationsWithReplacement<T> {
    type Item = Vec<T>;

    /// Returns the next combination and advances the internal iterator.
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let combo = self.get_current_combination();
        if self.move_to_next_position() == false {
            if self.all_sizes == false || self.move_to_next_set_size() == false {
                self.done = true;
            }
        }
        combo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations_iterable_to_sorted_set() {
        assert_eq!(vec![1, 2, 3, 4], iterable_to_sorted_set(vec![1, 2, 3, 4]));
        assert_eq!(vec![1, 2, 3, 4], iterable_to_sorted_set(1..5));
        assert_eq!(
            vec![1, 2, 3, 4].iter().collect::<Vec<&usize>>(),
            iterable_to_sorted_set(vec![2, 3, 1, 4].iter())
        );
        assert_eq!(
            vec![&1, &2, &3, &4],
            iterable_to_sorted_set(&vec![2, 1, 3, 1, 4, 2, 2, 3])
        );
    }

    #[test]
    fn test_combinations_all() {
        let combos = Combinations::all(vec![2, 4, 3, 1, 2, 2, 1].into_iter());
        assert_eq!(combos.elements, vec![1, 2, 3, 4]);
        assert_eq!(combos.positions, Vec::new());
        assert_eq!(combos.all_sizes, true);
        assert_eq!(combos.done, false);
    }

    #[test]
    fn test_combinations_w_rep_all() {
        let combos = CombinationsWithReplacement::all(vec![2, 4, 3, 1, 2, 2, 1].into_iter());
        assert_eq!(combos.elements, vec![1, 2, 3, 4]);
        assert_eq!(combos.positions, Vec::new());
        assert_eq!(combos.all_sizes, true);
        assert_eq!(combos.done, false);
    }

    #[test]
    fn test_combinations_of_size() {
        let combos = Combinations::of_size(vec![2, 4, 3, 1, 2, 2, 1].into_iter(), 3);
        assert_eq!(combos.elements, vec![1, 2, 3, 4]);
        assert_eq!(combos.positions, vec![0, 1, 2]);
        assert_eq!(combos.all_sizes, false);
        assert_eq!(combos.done, false);
    }

    #[test]
    fn test_combinations_w_rep_of_size() {
        let combos = CombinationsWithReplacement::of_size(vec![2, 4, 3, 1, 2, 2, 1].into_iter(), 3);
        assert_eq!(combos.elements, vec![1, 2, 3, 4]);
        assert_eq!(combos.positions, vec![0; 3]);
        assert_eq!(combos.all_sizes, false);
        assert_eq!(combos.done, false);
    }

    #[test]
    fn test_combinations_move_to_next_set_size() {
        let mut combos = Combinations::all(Vec::<i64>::new());
        assert_eq!(combos.positions, Vec::new());
        assert_eq!(combos.move_to_next_set_size(), false);
        let mut combos = Combinations::all(vec![1]);
        assert_eq!(combos.positions, Vec::new());
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0]);
        assert_eq!(combos.move_to_next_set_size(), false);
        let mut combos = Combinations::all(vec![1, 2, 3, 4]);
        assert_eq!(combos.positions, Vec::new());
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0]);
        combos.positions[0] = 4;
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0, 1]);
        combos.positions[0] = 5;
        combos.positions[1] = 2;
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0, 1, 2]);
        combos.positions[0] = 3;
        combos.positions[1] = 7;
        combos.positions[2] = 1;
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0, 1, 2, 3]);
        combos.positions[0] = 0;
        combos.positions[1] = 0;
        combos.positions[2] = 0;
        combos.positions[2] = 0;
        assert_eq!(combos.move_to_next_set_size(), false);
    }

    #[test]
    fn test_combinations_w_rep_move_to_next_set_size() {
        let mut combos = CombinationsWithReplacement::all(Vec::<i64>::new());
        assert_eq!(combos.positions, Vec::new());
        assert_eq!(combos.move_to_next_set_size(), false);
        let mut combos = CombinationsWithReplacement::all(vec![1]);
        assert_eq!(combos.positions, Vec::new());
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0]);
        assert_eq!(combos.move_to_next_set_size(), false);
        let mut combos = CombinationsWithReplacement::all(vec![1, 2, 3, 4]);
        assert_eq!(combos.positions, Vec::new());
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0]);
        combos.positions[0] = 4;
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0; 2]);
        combos.positions[0] = 5;
        combos.positions[1] = 2;
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0; 3]);
        combos.positions[0] = 3;
        combos.positions[1] = 7;
        combos.positions[2] = 1;
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0; 4]);
        combos.positions[0] = 0;
        combos.positions[1] = 0;
        combos.positions[2] = 0;
        combos.positions[2] = 0;
        assert_eq!(combos.move_to_next_set_size(), false);
    }

    #[test]
    fn test_combinations_move_to_next_position() {
        let mut combos = Combinations::of_size(Vec::<i64>::new(), 1);
        assert_eq!(combos.positions, vec![0]);
        assert_eq!(combos.move_to_next_position(), false);
        let mut combos = Combinations::of_size(vec![1], 1);
        assert_eq!(combos.positions, vec![0]);
        assert_eq!(combos.move_to_next_position(), false);
        let mut combos = Combinations::of_size(BTreeSet::from([1, 2, 3, 4]), 2);
        assert_eq!(combos.positions, vec![0, 1]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![2, 3]);
        assert_eq!(combos.move_to_next_position(), false);
        let mut combos = Combinations::of_size("abcd".chars(), 3);
        assert_eq!(combos.positions, vec![0, 1, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 1, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 2, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 2, 3]);
        assert_eq!(combos.move_to_next_position(), false);
    }

    #[test]
    fn test_combinations_w_rep_move_to_next_position() {
        let mut combos = CombinationsWithReplacement::of_size(Vec::<i64>::new(), 1);
        assert_eq!(combos.positions, vec![0]);
        assert_eq!(combos.move_to_next_position(), false);
        let mut combos = CombinationsWithReplacement::of_size(vec![1], 1);
        assert_eq!(combos.positions, vec![0]);
        assert_eq!(combos.move_to_next_position(), false);
        let mut combos = CombinationsWithReplacement::of_size(BTreeSet::from([1, 2, 3, 4]), 2);
        assert_eq!(combos.positions, vec![0, 0]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 1]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 1]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![2, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![2, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![3, 3]);
        assert_eq!(combos.move_to_next_position(), false);
        let mut combos = CombinationsWithReplacement::of_size("abcd".chars(), 3);
        assert_eq!(combos.positions, vec![0, 0, 0]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 0, 1]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 0, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 0, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 1, 1]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 1, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 1, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 2, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 2, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 3, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 1, 1]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 1, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 1, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 2, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 2, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 3, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![2, 2, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![2, 2, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![2, 3, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![3, 3, 3]);
        assert_eq!(combos.move_to_next_position(), false);
    }

    #[test]
    fn test_combinations_get_current_combination() {
        let mut combos = Combinations::of_size(vec![1, 1, 2, 3, 5, 8], 3);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 2, 3]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 2, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 2, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 3, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 3, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 5, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 3, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 3, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 5, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![3, 5, 8]));
        assert_eq!(combos.move_to_next_position(), false);
        combos.done = true;
        assert_eq!(combos.get_current_combination(), None);
    }

    #[test]
    fn test_combinations_w_rep_get_current_combination() {
        let mut combos = CombinationsWithReplacement::of_size(vec![1, 1, 2, 3, 5, 8], 3);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 1, 1]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 1, 2]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 1, 3]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 1, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 1, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 2, 2]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 2, 3]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 2, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 2, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 3, 3]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 3, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 3, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 5, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 5, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 8, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 2, 2]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 2, 3]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 2, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 2, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 3, 3]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 3, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 3, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 5, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 5, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 8, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![3, 3, 3]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![3, 3, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![3, 3, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![3, 5, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![3, 5, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![3, 8, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![5, 5, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![5, 5, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![5, 8, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![8, 8, 8]));
        assert_eq!(combos.move_to_next_position(), false);
        combos.done = true;
        assert_eq!(combos.get_current_combination(), None);
    }
}
