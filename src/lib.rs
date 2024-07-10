#![no_std]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
#![feature(exhaustive_patterns)]

use core::ops::{ControlFlow, FromResidual, Try};
use core::{convert::Infallible, ops::Residual};

use Result::*; // allows us to shadow the core::result::Result type

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub fn is_ok(&self) -> bool {
        // match self {
        //     Result::Ok(_) => true,
        //     Result::Err(_) => false,
        // }
        // is equivalent to:
        matches!(self, Ok(_))
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    pub fn ok(self) -> Option<T> {
        match self {
            Ok(t) => Some(t),
            Err(_) => None,
        }
    }

    pub fn err(self) -> Option<E> {
        match self {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    pub fn as_ref(&self) -> Result<&T, &E> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        }
    }

    pub fn as_mut(&mut self) -> Result<&mut T, &mut E> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        }
    }

    pub fn map<U, F>(self, op: F) -> Result<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Ok(t) => Ok(op(t)),
            Err(e) => Err(e), //  TODO: revisit with ? operator later
        }
    }

    pub fn map_err<U, F>(self, op: F) -> Result<T, U>
    where
        F: FnOnce(E) -> U,
    {
        match self {
            Err(e) => Err(op(e)), //  TODO: revisit with ? operator later
            Ok(t) => Ok(t),
        }
    }

    pub fn unwrap(self) -> T
    where
        E: core::fmt::Debug,
    {
        match self {
            Ok(t) => t,
            Err(e) => panic!("called Result::unwrap() on an Err value: {e:?}"),
        }
    }

    pub fn unwrap_err(self) -> E
    where
        T: core::fmt::Debug,
    {
        match self {
            Ok(t) => panic!("called Result::unwrap_err() on an Ok value: {t:?}"),
            Err(e) => e,
        }
    }

    pub fn and<U>(self, other: Result<U, E>) -> Result<U, E> {
        match self {
            Ok(_) => other,
            Err(e) => Err(e),
        }
    }

    pub fn and_then<U, F>(self, op: F) -> Result<U, E>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        match self {
            Ok(t) => op(t),
            Err(e) => Err(e),
        }
    }
}

impl<T, E> Try for Result<T, E> {
    type Output = T;
    type Residual = Result<Infallible, E>;

    fn from_output(output: Self::Output) -> Self {
        Ok(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Ok(t) => ControlFlow::Continue(t),
            Err(e) => ControlFlow::Break(Err(e)),
        }
    }
}

impl<T, E> Residual<T> for Result<Infallible, E> {
    type TryType = Result<T, E>;
}

impl<T, E, F> FromResidual<Result<Infallible, E>> for Result<T, F>
where
    F: From<E>,
{
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        let Err(e) = residual;

        Err(e.into())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_question_op() {
        fn might_err() -> Result<u32, &'static str> {
            Err("Errored out!")
        }

        fn nested_call() -> Result<u32, &'static str> {
            might_err()?;

            Err("whoops! :(")
        }

        // nested_call() must propagate might_err()'s Result::Err.
        assert_eq!(nested_call(), might_err());
    }
}
