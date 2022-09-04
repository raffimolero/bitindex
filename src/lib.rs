use std::{
    cell::UnsafeCell,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
struct BitsInner<'a> {
    fields: &'a mut u128,
    prev_index: u8,
    prev_out: bool,
}
impl<'a> BitsInner<'a> {
    fn update(&mut self) {
        *self.fields &= !(1 << self.prev_index);
        *self.fields |= (self.prev_out as u128) << self.prev_index;
    }

    fn observe(&mut self, index: u8) -> &mut bool {
        self.update();
        self.prev_index = index;
        self.prev_out = ((*self.fields >> index) & 1) == 1;
        &mut self.prev_out
    }
}
impl<'a> Drop for BitsInner<'a> {
    fn drop(&mut self) {
        self.observe(0);
    }
}
impl<'a> From<&'a mut u128> for BitsInner<'a> {
    fn from(fields: &'a mut u128) -> Self {
        Self {
            fields: fields,
            prev_index: 0,
            prev_out: false,
        }
    }
}

pub struct Bits<'a>(UnsafeCell<BitsInner<'a>>);
impl<'a> Bits<'a> {
    pub fn update(&self) {
        unsafe { &mut *self.0.get() }.update();
    }

    pub fn observe(&self, index: u8) -> &mut bool {
        unsafe { &mut *self.0.get() }.observe(index)
    }

    pub fn fields(&self) -> u128 {
        self.update();
        *unsafe { &*self.0.get() }.fields
    }
}
impl<'a> From<&'a mut u128> for Bits<'a> {
    fn from(fields: &'a mut u128) -> Self {
        Self(UnsafeCell::from(BitsInner::from(fields)))
    }
}
impl<'a> Index<u8> for Bits<'a> {
    type Output = bool;

    fn index(&self, index: u8) -> &Self::Output {
        self.observe(index)
    }
}
impl<'a> IndexMut<u8> for Bits<'a> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        self.observe(index)
    }
}
