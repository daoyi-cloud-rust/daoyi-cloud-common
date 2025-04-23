pub mod constants;

#[cfg(feature = "web")]
pub mod web;

#[cfg(feature = "gateway")]
pub mod gateway;

pub mod core;
pub mod module;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
