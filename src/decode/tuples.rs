use crate::PartialDecode;
use std::io::{self, Read};

macro_rules! arity_implementation {

    ($($type: ident), *) => {
        #[allow(non_snake_case)]
        impl <$($type), *> PartialDecode for ($($type), *)
        where
        $($type: PartialDecode), *
        {
            type Partial = ($($type::Partial), *);

            fn decode_from<R>(reader: &mut R) -> io::Result<($($type::Partial), *)>
            where
                R: Read
            {
                $(
                    let $type = $type::decode_from(reader)?;
                )*

                Ok(($($type), *))
            }

            fn complete(partial: ($($type::Partial), *)) -> ($($type), *) {
                let ($($type), *) = partial;

                $(
                    let $type = $type::complete($type);
                )*

                ($($type), *)
            }
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
