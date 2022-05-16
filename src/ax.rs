

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
/// // let fifth_axension = ax!(v);
///
/// // ERROR: Only accepts one identifier
/// //        If multiple axensions are what you need, see the 'axs' macro
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

#[cfg(test)]
mod tests {

    #[test]
    fn ax_works() {
        assert_eq!(ax!(x), 0);
        assert_eq!(ax!(y), 1);
        assert_eq!(ax!(z), 2);
        assert_eq!(ax!(w), 3);
    }

}


