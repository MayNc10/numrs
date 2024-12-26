use std::{io::Write, rc::Weak};

use arr_base::NDArrayBase;
use arr_ref::NDArrayRef;
use flags::Flags;

pub mod arr_base;
pub mod arr_ref;
pub mod flags;

pub trait NDArray<T: Clone> {
    fn flags(&self) -> &Flags;
    fn flags_mut(&mut self) -> &mut Flags;
    fn shape(&self) -> Vec<usize>;
    fn strides(&self) -> (); // TODO
    fn ndim(&self) -> usize;
    fn data(&self) -> &Vec<T>;
    fn data_mut(&mut self) -> &mut Vec<T>;
    fn size(&self) -> usize;
    fn itemsize(&self) -> usize {
        std::mem::size_of::<T>()
    }
    fn nbytes(&self) -> usize {
        self.size() * self.itemsize()
    }
    fn base(&self) -> Option<&NDArrayBase<T>>;
    fn base_mut(&mut self) -> Option<&mut NDArrayBase<T>>;
    fn transpose(&self) -> Box<dyn NDArray<T>>;
    fn real(&self) -> Box<dyn NDArray<i32>>; // need to be complex traits
    fn imag(&self) -> Box<dyn NDArray<i32>>;
    fn flat<'a>(&'a self) -> FlatIter<'a, T> 
    where Self: Sized
    {
        FlatIter { arr: self as &dyn NDArray<T>, step: 0 }
    }

    fn item(&self, idxs: &[usize]) -> Option<T>; // TODO: Add specific subtypes of item call?
    fn to_list(&self) -> List<T>;
    // to string is deprecated, not implementing
    fn to_bytes(&self, order: Option<Order>) -> Vec<u8>;
    fn to_file(&self, file: &mut dyn Write, sep: Option<&str>, format: &str);
    fn dump(&self, file: &mut dyn Write); // pickle
    fn dumps(&self) -> String; // pickle
    // should we even have multiple types? maybe we have a type for unsafe memcast
    // TODO: figure out how subtrait stuff should work here
    
    // endianness swap, figure out how to do this safely?
    fn byteswap(&mut self) -> Box<dyn NDArray<T>>;
    // byteswap no_mut?
    /// Despite the name, this is akin to a clone function, with the option to change the order
    fn copy(&self, order: Option<Order>) -> NDArrayBase<T>;
    fn view(&self) -> (); // This will be a hard one to make safe, and requires a lot of consideration

    fn setflags(&mut self, write: bool, align: bool, write_back_copy: bool) -> bool; // returns a bool based on correctness of args
    fn fill(&self, val: T);
    fn reshape(&mut self, shape: Vec<usize>, order: Option<Order>, copy: bool) -> Box<dyn NDArray<T>>;


}

impl<T: Clone> dyn NDArray<T> 
where Self: Sized,
{
    pub fn as_type<This: NDArray<T> + Sized, U: From<T>>(this: &This, order: Option<Order>) -> NDArrayBase<U> 
    {
        let iter = this.flat();
        todo!()
    }

    /// This function is unsafe because it can create invalid instances of types, 
    pub fn getfield<This: NDArray<T> + Sized, U: From<T>>(this: &This, offset: usize) -> Option<NDArrayRef<U>> 
    {
        let iter = this.flat();
        todo!()
    }
}

pub enum List<T> {
    Vec (Vec<List<T>>),
    Scalar (T)
}

pub enum Order {
    C,
    Fortran,
    Any,
    Close, 
    FortranPriority,
}

pub struct FlatIter<'a, T> {
    arr: &'a dyn NDArray<T>,
    step: usize,
}

impl<'a, T> Iterator for FlatIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        todo!()
    }
}


