//!
//! A collection of macros to generate usize values for indexing data structures
//!

#![no_std]


///
/// Converts an identifier _x_, _y_, _z_ or _w_ to a `usize` value.
///
/// Using any identifier apart from the above or multiple identifiers will result in a compile time error.
///
/// It is recommended to use parentheses when calling this macro for clarity.
///
/// # Possible Variations
///
/// ```
/// # #[macro_use] extern crate axmac; fn main() {
/// # use axmac::ax;
/// let first: usize = ax!(x);
/// let second = ax!(y);
/// let third  = ax!(z);
/// let fourth = ax!(w);
///
/// assert_eq!(first,  0);
/// assert_eq!(second, 1);
/// assert_eq!(third,  2);
/// assert_eq!(fourth, 3);
///
/// // ERROR: Only allowed to use one of x, y, z or w
/// // let fifth_dimension = ax!(v);
///
/// // ERROR: Only accepts one identifier
/// //        If multiple axes are what you need, see the 'axs' macro
/// // let third_and_fourth = ax!(z, w);
/// # }
/// ```
///
#[macro_export]
macro_rules! ax {

    (x) => { 0usize };
    (y) => { 1usize };
    (z) => { 2usize };
    (w) => { 3usize };

}


///
/// Converts an array of identifiers _x_, _y_, _z_ or _w_ to an array of `usize` values
///
/// Using any identifier or expression apart from the above will result in a compile time error
///
/// It is recommended to use square brackets when calling this macro for clarity
///
/// # Possible Variations
///
/// ```
/// # #[macro_use] extern crate axmac; fn main() {
/// # use axmac::axs;
/// // Explicitly specifying items in array
/// let arr =  axs![x, y, z, w];
/// assert_eq!(arr, [0, 1, 2, 3]);
///
/// // Repeat specified item N times
/// let arr =  axs![w; 5];
/// assert_eq!(arr, [3, 3, 3, 3, 3]);
/// # }
/// ```
///
/// Using identifiers multiple times is allowed, this is only a more readable way to create arrays after
/// all
///
/// ```
/// # #[macro_use] extern crate axmac; fn main() {
/// # use axmac::axs;
/// let index_arr =  axs![x,x, z,z, y,y];
/// assert_eq!(index_arr, [0,0, 2,2, 1,1]);
/// # }
/// ```
///
#[macro_export]
macro_rules! axs {

    // [x, x, w, z, y, z]
    ( $( $d:ident ), * ) => { [ $( ax!($d), )* ] };

    // [z; 3]
    ( $d:ident; $i:expr ) => { [ax!($d); $i] };

}


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
/// assert_eq!(axr!(x..z), 0..2);
///
/// // RangeInclusive with identifiers
/// assert_eq!(axr!(y..=w), 1..=3);
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
/// assert_eq!(axr!(x..10), 0..10);
///
/// // RangeInclusive with identifier and expression
/// assert_eq!(axr!(x..=7), 0..=7);
///
/// // Range with expression and identifier
/// //  The parentheses around the expression are compulsory
/// assert_eq!(axr!((0)..z), 0..2);
///
/// // RangeInclusive with expression and identifier
/// //  The parentheses around the expression are compulsory
/// assert_eq!(axr!((1)..=w), 1..=3);
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
mod tests {

    #[cfg(test)]
    mod ax {
        #[test]
        fn it_works() {
            assert_eq!(ax!(x), 0);
            assert_eq!(ax!(y), 1);
            assert_eq!(ax!(z), 2);
            assert_eq!(ax!(w), 3);
        }
    }

    #[cfg(test)]
    mod axs {
        #[test]
        fn it_works() {
            assert_eq!(axs![x,y,z,w], [0,1,2,3]);
            assert_eq!(axs![x,z,y],   [0,2,1]);
            assert_eq!(axs![x,y,x,y], [0,1,0,1]);
        }
    }


    #[cfg(test)]
    mod axr {

        #[test]
        fn ident_to_ident_works() {
            let arr = [0,1,2,3,4,5,6,7,8,9];
            let slice = &arr[axr![x..z]];
            assert_eq!(*slice, [0,1]);
        }
        #[test]
        fn ident_to_eq_ident_works() {
            let arr = [0,1,2,3,4,5,6,7,8,9];
            let slice = &arr[axr![y..=w]];
            assert_eq!(*slice, [1,2,3]);
        }

        #[test]
        fn ident_to_expr_works() {
            let arr = [0,1,2,3,4,5,6,7,8,9];
            let slice = &arr[axr![y..9]];
            assert_eq!(*slice, [1,2,3,4,5,6,7,8]);
        }
        #[test]
        fn ident_to_eq_expr_works() {
            let arr = [0,1,2,3,4,5,6,7,8,9];
            let slice = &arr[axr![x..=5]];
            assert_eq!(*slice, [0,1,2,3,4,5]);
        }

        #[test]
        fn inf_to_ident_works() {
            let arr = [0,1,2,3,4,5,6,7,8,9];
            let slice = &arr[axr![..w]];
            assert_eq!(*slice, [0,1,2]);
        }
        #[test]
        fn inf_to_eq_ident_works() {
            let arr = [0,1,2,3,4,5,6,7,8,9];
            let slice = &arr[axr![..=w]];
            assert_eq!(*slice, [0,1,2,3]);
        }

        #[test]
        fn ident_to_inf_works() {
            let arr = [0,1,2,3,4,5,6,7,8,9];
            let slice = &arr[axr![x..]];
            assert_eq!(*slice, arr);
        }

        #[test]
        fn expr_to_ident_works() {
            let arr = [0,1,2,3,4];
            let slice = &arr[axr!((0)..z)];
            assert_eq!(*slice, [0,1]);

            let expr = 1usize;
            let slice = &arr[axr!((expr)..w)];
            assert_eq!(*slice, [1, 2]);
        }

        #[test]
        fn expr_to_eq_ident_works() {
            let arr = [0,1,2,3,4];
            let slice = &arr[axr!((0)..=z)];
            assert_eq!(*slice, [0,1,2]);

            let expr = 1usize;
            let slice = &arr[axr!((expr)..=w)];
            assert_eq!(*slice, [1,2,3]);
        }

    }


}
