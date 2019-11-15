#[macro_use]
extern crate failure;

mod api;
pub mod models;

mod riot_api;

pub use self::api::*;
pub use self::riot_api::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
