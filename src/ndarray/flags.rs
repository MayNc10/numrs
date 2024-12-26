macro_rules! access {
    ($attrib:ident) => {
        pub fn $attrib(&self) -> bool {
            self.$attrib
        }
    };
}


pub(super) struct Flags {
    c_contiguous: bool,
    f_contiguous: bool,
    owndata: bool,
    writeable: bool,
    aligned: bool,
    write_back_if_copy: bool,
}

/// This implementation breaks the spec just a bit
/// Because of Rust being a stricter (better) language than python
/// The writeable and aligned modification functions should be called from ndarray
/// Additionally, there are no index operations provided, because calling the functions is better practice
impl Flags {
    pub(super) fn new(c_contiguous: bool, f_contiguous: bool, owndata: bool, 
        writeable: bool, aligned: bool, write_back_if_copy: bool) -> Flags 
    {
        Flags { c_contiguous, f_contiguous, owndata, writeable, aligned, write_back_if_copy }
    }
    
    access!{c_contiguous}
    access!{f_contiguous}
    access!{owndata}
    access!{writeable}
    access!{aligned}
    access!{write_back_if_copy}
    
    // i'm breaking the spec here to make the library better
    pub fn set_nonwriteback(&mut self) {
        self.write_back_if_copy = false;
    }

    pub fn fnc(&self) -> bool {
        self.f_contiguous && !self.c_contiguous
    }

    pub fn forc(&self) -> bool {
        self.f_contiguous || self.c_contiguous
    }

    pub fn behaved(&self) -> bool {
        self.aligned && self.writeable
    }

    pub fn carray(&self) -> bool {
        self.behaved() && self.c_contiguous
    }

    pub fn farray(&self) -> bool {
        self.behaved() && self.fnc()
    }
}   

