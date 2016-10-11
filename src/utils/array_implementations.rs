//! Macro, to implement the trait for all fixed-sized arrays

macro_rules! implement_for_arrays {
    ($trait_ti: ident) => {
        impl<E> $trait_ti<E> for [E; 0] {}
        impl<E> $trait_ti<E> for [E; 1] {}
        impl<E> $trait_ti<E> for [E; 2] {}
        impl<E> $trait_ti<E> for [E; 3] {}
        impl<E> $trait_ti<E> for [E; 4] {}
        impl<E> $trait_ti<E> for [E; 5] {}
        impl<E> $trait_ti<E> for [E; 6] {}
        impl<E> $trait_ti<E> for [E; 7] {}
        impl<E> $trait_ti<E> for [E; 8] {}
        impl<E> $trait_ti<E> for [E; 9] {}
        impl<E> $trait_ti<E> for [E; 10] {}
        impl<E> $trait_ti<E> for [E; 11] {}
        impl<E> $trait_ti<E> for [E; 12] {}
        impl<E> $trait_ti<E> for [E; 13] {}
        impl<E> $trait_ti<E> for [E; 14] {}
        impl<E> $trait_ti<E> for [E; 15] {}
        impl<E> $trait_ti<E> for [E; 16] {}
        impl<E> $trait_ti<E> for [E; 17] {}
        impl<E> $trait_ti<E> for [E; 18] {}
        impl<E> $trait_ti<E> for [E; 19] {}
        impl<E> $trait_ti<E> for [E; 20] {}
        impl<E> $trait_ti<E> for [E; 21] {}
        impl<E> $trait_ti<E> for [E; 22] {}
        impl<E> $trait_ti<E> for [E; 23] {}
        impl<E> $trait_ti<E> for [E; 24] {}
        impl<E> $trait_ti<E> for [E; 25] {}
        impl<E> $trait_ti<E> for [E; 26] {}
        impl<E> $trait_ti<E> for [E; 27] {}
        impl<E> $trait_ti<E> for [E; 28] {}
        impl<E> $trait_ti<E> for [E; 29] {}
        impl<E> $trait_ti<E> for [E; 30] {}
        impl<E> $trait_ti<E> for [E; 31] {}
        impl<E> $trait_ti<E> for [E; 32] {}
    };
}
