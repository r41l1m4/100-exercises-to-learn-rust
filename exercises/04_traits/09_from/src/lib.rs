// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

// WrappingU32::from(T);
impl<T> From<T> for WrappingU32 {
    fn from(value: T) -> Self {
        Self {
            value,
        }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
