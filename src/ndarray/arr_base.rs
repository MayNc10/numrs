use super::flags::Flags;

pub struct NDArrayBase<T> {
    flags: Flags,
    shape: Vec<usize>,
    strides: (), 
    data: Vec<T>,
    size: usize,
}