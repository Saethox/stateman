use std::fmt;

use better_any::Tid;

/// Trait to represent any type that is `Send + 'a`.
///
/// A resource is a data slot which lives in the [Resources][crate::Resources] can only be accessed
/// according to Rust's typical borrowing model (one writer xor multiple
/// readers).
#[cfg(not(feature = "debug"))]
pub trait Resource<'a>: Tid<'a> + Send {
    fn type_name(&self) -> TypeNameLit;
}

#[cfg(not(feature = "debug"))]
impl<'a, T> Resource<'a> for T
where
    T: Tid<'a> + Send,
{
    fn type_name(&self) -> TypeNameLit {
        TypeNameLit(std::any::type_name::<T>())
    }
}

/// Trait to represent any type that is `Send + 'a`.
///
/// A resource is a data slot which lives in the [Resources][crate::Resources] can only be accessed
/// according to Rust's typical borrowing model (one writer xor multiple
/// readers).
#[cfg(feature = "debug")]
pub trait Resource<'a>: Tid<'a> + Send + std::fmt::Debug {
    fn type_name(&self) -> TypeNameLit;
}

#[cfg(feature = "debug")]
impl<'a, T> Resource<'a> for T
where
    T: Tid<'a> + std::fmt::Debug + Send + 'a,
{
    fn type_name(&self) -> TypeNameLit {
        TypeNameLit(std::any::type_name::<T>())
    }
}

pub struct TypeNameLit(&'static str);

impl fmt::Debug for TypeNameLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
