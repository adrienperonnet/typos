use num_traits::{Bounded, CheckedAdd, Zero};
use std::cmp::{min, Ord, Ordering};
use std::ops::Add;

/*
In order to derive the Copy trait required by pathfinding api we use fixed size array for storage
Increasing this value with increase the memory required by the program but will support more hops.
*/
pub const MAX_DIMENSION: usize = 20;

/// A metric data for Path that supports different layers
#[derive(Debug, Copy, Clone, Eq)]
pub struct PathMultiCost<U> {
    data: [U; MAX_DIMENSION],
}

impl<U: Zero + PartialEq + Copy> PathMultiCost<U> {
    pub fn get_cost(self) -> Vec<(U, usize)> {
        self.data
            .iter()
            .enumerate()
            .filter(|&u| *u.1 != U::zero())
            .map(|(k, v)| (*v, MAX_DIMENSION - k))
            .collect::<Vec<(U, usize)>>()
    }
}

impl<U: Zero + Copy> PathMultiCost<U> {
    pub fn new(cost: U, dimension: usize) -> PathMultiCost<U> {
        let mut data = [U::zero(); MAX_DIMENSION];
        data[min(MAX_DIMENSION - 1, MAX_DIMENSION - 1 - dimension)] = cost;
        return PathMultiCost { data };
    }
}

impl<U: Bounded + Copy + Zero> Bounded for PathMultiCost<U> {
    fn min_value() -> Self {
        return PathMultiCost::new(U::min_value(), 0);
    }
    fn max_value() -> Self {
        return PathMultiCost::new(U::max_value(), MAX_DIMENSION - 1);
    }
}

impl<U: PartialEq> PartialEq for PathMultiCost<U> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<U: Ord> Ord for PathMultiCost<U> {
    fn cmp(&self, other: &Self) -> Ordering {
        for kv in self.data.iter().zip(other.data.iter()) {
            match kv.0.cmp(kv.1) {
                Ordering::Equal => (),
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
            }
        }
        return Ordering::Equal;
    }
}

impl<U: PartialOrd> PartialOrd for PathMultiCost<U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.data.partial_cmp(&other.data);
    }
}

impl<U: Zero + Copy + CheckedAdd + Bounded> Add for PathMultiCost<U> {
    type Output = PathMultiCost<U>;

    fn add(self, rhs: PathMultiCost<U>) -> Self::Output
    where
        U: CheckedAdd,
    {
        let mut array = self.data;
        rhs.data
            .iter()
            .enumerate()
            .for_each(|(i, e)| match e.checked_add(&array[i]) {
                None => array[i] = U::max_value(),
                Some(s) => array[i] = s,
            });
        return PathMultiCost::<U> { data: array };
    }
}

impl<U: Zero + Copy + Bounded + CheckedAdd> Zero for PathMultiCost<U> {
    fn zero() -> Self {
        return PathMultiCost::new(U::zero(), 0);
    }
    fn is_zero(&self) -> bool {
        self.data
            .iter()
            .rev() //reverse to speedup because it's more likely that the last elements are not empty
            .all(|u| u.is_zero())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_cost() {
        assert_eq!(cost(&[2]).get_cost(), vec![(2, 1)]);
        assert_eq!(cost(&[5, 3]).get_cost(), vec![(5, 2), (3, 1)]);
        assert_eq!(cost(&[1, 5, 3]).get_cost(), vec![(1, 3), (5, 2), (3, 1)])
    }

    #[test]
    fn zero() {
        assert!(cost(&[0, 0, 0, 0, 0]).is_zero());
        assert!(!cost(&[0, 1, 0]).is_zero());
        assert!(!cost(&[1, 2, 3]).is_zero());
    }

    #[test]
    fn equality() {
        assert_ne!(cost(&[1, 2, 3]), cost(&[0, 1, 0]));
        assert_eq!(cost(&[0, 1, 0]), cost(&[0, 1, 0]));
        assert_eq!(cost(&[1, 0]), cost(&[0, 0, 0, 1, 0]));
    }

    #[test]
    fn zero_identity_element() {
        assert_eq!(PathMultiCost::zero() + cost(&[1, 2, 3]), cost(&[1, 2, 3]));
    }

    #[test]
    fn sum_commutative() {
        assert_eq!(
            cost(&[1, 2, 3]) + cost(&[4, 5, 6]),
            cost(&[4, 5, 6]) + cost(&[1, 2, 3])
        );
    }

    #[test]
    fn sum_associative() {
        assert_eq!(
            cost(&[1, 2, 3]) + (cost(&[4, 5, 6]) + cost(&[0, 1, 2])),
            (cost(&[1, 2, 3]) + cost(&[4, 5, 6])) + cost(&[0, 1, 2])
        );
    }

    #[test]
    fn sum() {
        assert_eq!(cost(&[1]) + cost(&[2]), cost(&[3]));
        assert_eq!(cost(&[1, 0]) + cost(&[5, 0]), cost(&[6, 0]));
        assert_eq!(cost(&[1, 2, 3]) + cost(&[3, 2, 1]), cost(&[4, 4, 4]));
    }

    #[test]
    fn ordering_prefer_high_dimension() {
        assert!(cost(&[0, 0, 2]) > cost(&[0, 0, 1]));
        assert!(cost(&[0, 2, 0]) > cost(&[0, 0, 5]));
        assert!(cost(&[3, 0, 0]) > cost(&[2, 71, 88]));
    }

    #[test]
    fn subadditivity() {
        //f(x+y)<=f(x)+f(y)
        assert!(cost(&[3, 5, 4]) <= cost(&[2, 5, 4]) + cost(&[1, 5, 4]));
        assert!(cost(&[3, 5, 4]) <= cost(&[2, 3, 1]) + cost(&[1, 2, 3]));
    }

    extern crate quickcheck;

    use quickcheck::empty_shrinker;
    use quickcheck::quickcheck;

    fn from_vec<U: Zero + Copy>(v: Vec<U>) -> PathMultiCost<U> {
        let mut array = [U::zero(); MAX_DIMENSION];
        v.iter().take(MAX_DIMENSION).enumerate().for_each(|(i, u)| {
            array[i] = *u;
        });
        return PathMultiCost { data: array };
    }

    impl<U: quickcheck::Arbitrary + Copy + Zero + Copy + Bounded + CheckedAdd> quickcheck::Arbitrary
        for PathMultiCost<U>
    {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> PathMultiCost<U> {
            let input: Vec<U> = quickcheck::Arbitrary::arbitrary(g);
            return from_vec(input);
        }

        fn shrink(&self) -> Box<Iterator<Item = Self>> {
            match self.is_zero() {
                true => empty_shrinker(),
                false => Box::new(self.data.to_vec().shrink().map(from_vec)),
            }
        }
    }

    use super::*;

    quickcheck! {
        //Add some property-based testing on
        fn equal_function_prop(a: PathMultiCost<u8>) -> bool {
            a == a
        }

        //Check monoid properties (totality,identity,associativity)
        fn sum_zero_prop(a: PathMultiCost<u8>, b: PathMultiCost<u8>) -> bool {
            b == PathMultiCost::zero() || a + b != a
        }

        fn sum_add_zero_prop(a: PathMultiCost<u8>) -> bool {
            a + PathMultiCost::zero() == a
        }

        fn sum_associative_prop(a: PathMultiCost<u8>, b: PathMultiCost<u8>, c: PathMultiCost<u8>) -> bool {
            a + (b + c) == (a + b) + c
        }

        fn sum_commutative_prop(a: PathMultiCost<u8>, b: PathMultiCost<u8>) -> bool {
            a + b == b + a
        }

        fn bounded_prop(a: PathMultiCost<u8>) -> bool {
            a >= PathMultiCost::min_value() && a <= PathMultiCost::max_value()
        }

        //triangle inequality
        fn subadditivity_prop(a: u8, b: u8, c: u8, y: u8) -> bool {
            //f(x+y)<=f(x)+f(y)
            cost(&[a + y, b, c]) <= cost(&[a, b, c]) + cost(&[y, 0, 0]) &&
                cost(&[a, b + y, c]) <= cost(&[a, b, c]) + cost(&[0, y, 0]) &&
                cost(&[a, b, c + y]) <= cost(&[a, b, c]) + cost(&[0, 0, y]) &&
                cost(&[a + y, b + y, c + y]) <= cost(&[a, b, c]) + cost(&[y, y, y])
        }

        //total ordering
        fn antisymmetry_prop(a: PathMultiCost<u8>, b: PathMultiCost<u8>) -> bool {
            if a >= b && a <= b { a == b } else { true }
        }

        fn transitivity_prop(a: PathMultiCost<u8>, b: PathMultiCost<u8>, c: PathMultiCost<u8>) -> bool {
            if a <= b && b <= c { a <= c } else { true }
        }

        //total
        fn connexity_prop(a: PathMultiCost<u8>, b: PathMultiCost<u8>) -> bool {
            a <= b || b <= a
        }

        //reflexity
         fn reflexity_prop(a: PathMultiCost<u8>) -> bool {
            a <= a
        }

        //PathMultiCost<u8> is isotone
         fn isotone_prop(a: PathMultiCost<u8>, b: PathMultiCost<u8>, c: PathMultiCost<u8>) -> bool {
            if a <= b {a + c <= b + c && c + a <= c + b } else {true}
        }
    }

    fn cost(input: &[u8]) -> PathMultiCost<u8> {
        let mut data = [0; MAX_DIMENSION];
        input
            .iter()
            .enumerate()
            .for_each(|(i, _)| (data[MAX_DIMENSION - i - 1] = input[input.len() - i - 1]));
        return PathMultiCost { data };
    }
}
