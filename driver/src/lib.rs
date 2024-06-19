pub mod aqara_fp2;

pub use aqara_fp2::{AqaraFP2Discovery, AqaraFP2Device, AqaraFP2Driver};

pub fn add(left: usize, right: usize) -> usize {
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
