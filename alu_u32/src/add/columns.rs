use core::borrow::{Borrow, BorrowMut};
use core::mem::{size_of, transmute};
use valida_derive::AlignedBorrow;
use valida_machine::Word;
use valida_util::indices_arr;

/// Representing columns for the addition of two 32-bit words. The output is
/// witnessed, and the carry is computed. The `is_real` field is used to
/// indicate whether the output is a real row or is a padding row(need to confirm).
#[derive(AlignedBorrow, Default)]
pub struct Add32Cols<T> {
    pub input_1: Word<T>,
    pub input_2: Word<T>,

    pub carry: [T; 3],

    /// Witnessed output
    pub output: Word<T>,

    pub is_real: T,
}

/// The size of the `Add32Cols<u8>` struct in bytes. It should be 16 bytes.
pub const NUM_ADD_COLS: usize = size_of::<Add32Cols<u8>>();

/// The column map for the `Add32Cols` struct.
pub const ADD_COL_MAP: Add32Cols<usize> = make_col_map();

/// Method to generate the column map for the `Add32Cols` struct.
const fn make_col_map() -> Add32Cols<usize> {
    let indices_arr = indices_arr::<NUM_ADD_COLS>();
    unsafe { transmute::<[usize; NUM_ADD_COLS], Add32Cols<usize>>(indices_arr) }
}
