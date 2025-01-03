// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.
use std::ops::Add;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

// trait Add<RHS = Self> {
//     type Output;
//
//     fn add(&self, n: RHS) -> Self::Output;
// }

impl Add<WrappingU32> for WrappingU32 {
    type Output = WrappingU32;

    fn add(self, n: WrappingU32) -> Self::Output {
        let new_value = self.value.wrapping_add(n.value);
        WrappingU32::new(new_value)

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
