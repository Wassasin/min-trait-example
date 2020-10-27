use generic_array::{typenum::Unsigned, ArrayLength, GenericArray};

fn convert_generic_array<N: ArrayLength<u8> + Unsigned>(
    array: GenericArray<u8, N>,
) -> [u8; <N as Unsigned>::USIZE] {
    todo!()
}
