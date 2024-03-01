#[allow(missing_debug_implementations)]

pub struct PrivateMarker;

macro_rules! private_decl {
    () => {
        fn __rayon_private__(&self) -> crate::private::PrivateMarker;
    };
}

macro_rules! private_impl {
    () => {
        fn __rayon_private__(&self) -> crate::private::PrivateMarker {
            crate::private::PrivateMarker
        }
    };
}
