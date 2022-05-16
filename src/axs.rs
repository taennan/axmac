
use crate::ax;

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

#[cfg(test)]
mod tests {
    
    #[test]
    fn axs_works() {
        assert_eq!(axs![x,y,z,w], [0,1,2,3]);
        assert_eq!(axs![x,z,y],   [0,2,1]);
        assert_eq!(axs![x,y,x,y], [0,1,0,1]);
    }
    
}