use crate::{Encode, StableEncode};
use std::io::{self, Write};

macro_rules! arity_implementation {
    ($($type: ident), *) => {
        impl <$($type), *> Encode for ($($type), *)
        where
            $($type: Encode), *
        {
            fn encode_unstable_into<W>(&self, writer: &mut W) -> io::Result<()>
            where
                W: Write,
            {
                #[allow(non_snake_case)]
                let ($($type), *) = self;

                $(
                    $type.encode_unstable_into(writer)?;
                )*

                Ok(())
            }
        }

        impl <$($type), *> StableEncode for ($($type), *)
        where
            $($type: StableEncode), *
        {
        }
    };
}

arity_implementation!(A, B);
arity_implementation!(A, B, C);
arity_implementation!(A, B, C, D);
arity_implementation!(A, B, C, D, E);
arity_implementation!(A, B, C, D, E, F);
arity_implementation!(A, B, C, D, E, F, G);
arity_implementation!(A, B, C, D, E, F, G, H);
arity_implementation!(A, B, C, D, E, F, G, H, I);
arity_implementation!(A, B, C, D, E, F, G, H, I, J);
arity_implementation!(A, B, C, D, E, F, G, H, I, J, K);
arity_implementation!(A, B, C, D, E, F, G, H, I, J, K, L);
arity_implementation!(A, B, C, D, E, F, G, H, I, J, K, L, M);
arity_implementation!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
arity_implementation!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
arity_implementation!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
