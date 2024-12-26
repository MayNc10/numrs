use std::{marker::PhantomData, rc::Weak};

use super::arr_base::NDArrayBase;

pub struct NDArrayRef<T> {
    // TODO
    // this struct should hold a ref to the **data** of a base arrray
    // but can change other aspects including the stride and such
    pht: PhantomData<T>,
}
