
use crate::ax;

///
/// Converts a range of identifiers and/or `usize` expressions to a range of `usize` values
///
/// Using any identifiers apart from _x_, _y_, _z_ or _w_ will result in a compile time error
///
///  It is recommended to use parentheses when calling this macro for clarity
///
/// # Possible Variations
///
/// ```
/// # #[macro_use] extern crate axmac; fn main() {
/// # use axmac::axr;
/// // PLEASE NOTE!
/// //  x = 0usize
/// //  y = 1
/// //  z = 2
/// //  w = 3
///
/// // Range with identifiers
/// assert_eq!(axr!(x..z), 0..2usize);
///
/// // RangeInclusive with identifiers
/// assert_eq!(axr!(y..=w), 1..=3usize);
///
/// // RangeTo with identifiers
/// assert_eq!(axr!(..z), ..2);
///
/// // RangeToInclusive with identifiers
/// assert_eq!(axr!(..=w), ..=3);
///
/// // RangeFrom with identifier
/// assert_eq!(axr!(y..), 1..);
///
/// // Range with identifier and expression
/// assert_eq!(axr!(x..10), 0..10usize);
///
/// // RangeInclusive with identifier and expression
/// assert_eq!(axr!(x..=7), 0..=7usize);
///
/// // Range with expression and identifier
/// //  The parentheses around the expression are compulsory
/// assert_eq!(axr!((0)..z), 0..2usize);
///
/// // RangeInclusive with expression and identifier
/// //  The parentheses around the expression are compulsory
/// assert_eq!(axr!((1)..=w), 1..=3usize);
/// # }
/// ```
///
#[macro_export]
macro_rules! axr {

    // Ident to Ident
    //  Range x..w
    ( $a:ident..$b:ident ) => { ax!($a)..ax!($b) };
    //  RangeInclusive y..=z
    ( $a:ident..=$b:ident ) => { ax!($a)..=ax!($b) };

    // Ident to Expr
    //  Range z..6
    ( $a:ident..$b:expr ) => { ax!($a)..$b };
    //  RangeInclusive w..=9
    ( $a:ident..=$b:expr ) => { ax!($a)..=$b };

    // Inf to Ident
    //  RangeTo ..w
    ( ..$a:ident ) => { ..ax!($a) };
    //  RangeToInclusive ..=z
    ( ..=$a:ident ) => { ..=ax!($a) };

    // Ident to Inf
    //  RangeFrom x..
    ( $a:ident.. ) => { ax!($a).. };

    // Expr to Ident
    //  Range (0)..z
    ( ($a:expr)..$b:ident )  => { $a..ax!($b) };
    // RangeInclusive (1)..=w
    ( ($a:expr)..=$b:ident )  => { $a..=ax!($b) };

}

#[cfg(test)]
mod test {
    
    #[test]
    fn axr_ident_to_ident_works() {
        let arr = [0,1,2,3,4,5,6,7,8,9];
        let slice = &arr[axr![x..z]];
        assert_eq!(*slice, [0,1]);
    }
    #[test]
    fn axr_ident_to_eq_ident_works() {
        let arr = [0,1,2,3,4,5,6,7,8,9];
        let slice = &arr[axr![y..=w]];
        assert_eq!(*slice, [1,2,3]);
    }

    #[test]
    fn axr_ident_to_expr_works() {
        let arr = [0,1,2,3,4,5,6,7,8,9];
        let slice = &arr[axr![y..9]];
        assert_eq!(*slice, [1,2,3,4,5,6,7,8]);
    }
    #[test]
    fn axr_ident_to_eq_expr_works() {
        let arr = [0,1,2,3,4,5,6,7,8,9];
        let slice = &arr[axr![x..=5]];
        assert_eq!(*slice, [0,1,2,3,4,5]);
    }

    #[test]
    fn axr_inf_to_ident_works() {
        let arr = [0,1,2,3,4,5,6,7,8,9];
        let slice = &arr[axr![..w]];
        assert_eq!(*slice, [0,1,2]);
    }
    #[test]
    fn axr_inf_to_eq_ident_works() {
        let arr = [0,1,2,3,4,5,6,7,8,9];
        let slice = &arr[axr![..=w]];
        assert_eq!(*slice, [0,1,2,3]);
    }

    #[test]
    fn axr_ident_to_inf_works() {
        let arr = [0,1,2,3,4,5,6,7,8,9];
        let slice = &arr[axr![x..]];
        assert_eq!(*slice, arr);
    }

    #[test]
    fn axr_expr_to_ident_works() {
        let arr = [0,1,2,3,4];
        let slice = &arr[axr!((0)..z)];
        assert_eq!(*slice, [0,1]);

        let expr = 1usize;
        let slice = &arr[axr!((expr)..w)];
        assert_eq!(*slice, [1, 2]);
    }

    #[test]
    fn axr_expr_to_eq_ident_works() {
        let arr = [0,1,2,3,4];
        let slice = &arr[axr!((0)..=z)];
        assert_eq!(*slice, [0,1,2]);

        let expr = 1usize;
        let slice = &arr[axr!((expr)..=w)];
        assert_eq!(*slice, [1,2,3]);
    }
    
}