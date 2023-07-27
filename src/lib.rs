macro_rules! flat_mod {
    ($($i:ident),+) => {
        $(
            mod $i;
            pub use $i::*;
        )+
    };
}

#[path = "bindings.rs"]
pub mod sys;

flat_mod! {
    result,
    context
}
