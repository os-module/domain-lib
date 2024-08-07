use core::{
    alloc::Layout,
    fmt::{Debug, Formatter},
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

use super::{CustomDrop, RRef, RRefable, SharedData, TypeIdentifiable};

pub struct RRefVec<T>
where
    T: 'static + RRefable + Copy + TypeIdentifiable,
{
    data: RRef<T>,
    size: usize,
}
unsafe impl<T> RRefable for RRefVec<T> where T: 'static + RRefable + Copy + TypeIdentifiable {}
unsafe impl<T> Send for RRefVec<T> where T: 'static + RRefable + Copy + TypeIdentifiable {}

impl<T> RRefVec<T>
where
    T: 'static + RRefable + Copy + TypeIdentifiable,
{
    pub fn new(initial_value: T, size: usize) -> Self {
        let layout = Layout::array::<T>(size).unwrap();
        let data = unsafe { RRef::new_with_layout(initial_value, layout) };
        let mut vec = Self { data, size };
        vec.as_mut_slice().fill(initial_value);
        vec
    }
    #[allow(clippy::uninit_assumed_init)]
    pub fn from_slice(slice: &[T]) -> Self {
        let size = slice.len();
        let layout = Layout::array::<T>(size).unwrap();
        let data = unsafe { RRef::new_with_layout(MaybeUninit::uninit().assume_init(), layout) };
        let mut vec = Self { data, size };
        for (dest, src) in vec.as_mut_slice().iter_mut().zip(slice) {
            *dest = *src;
        }
        vec
    }
    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(&*self.data, self.size) }
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(&mut *self.data, self.size) }
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T: RRefable + Copy + TypeIdentifiable> Index<usize> for RRefVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T: RRefable + Copy + TypeIdentifiable> IndexMut<usize> for RRefVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl<T> Debug for RRefVec<T>
where
    T: 'static + RRefable + Copy + TypeIdentifiable + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RRefVec")
            .field("data", &self.data)
            .field("size", &self.size)
            .finish()
    }
}

impl<T: RRefable + Copy + TypeIdentifiable> Drop for RRefVec<T> {
    fn drop(&mut self) {
        log::warn!("<drop> for RRefVec");
    }
}

impl<T: RRefable + Copy + TypeIdentifiable> CustomDrop for RRefVec<T> {
    fn custom_drop(&mut self) {
        log::warn!("<custom_drop> for RRefVec");
        self.data.custom_drop();
    }
}

impl<T: RRefable + Copy + TypeIdentifiable> SharedData for RRefVec<T> {
    fn move_to(&self, new_domain_id: u64) -> u64 {
        self.data.move_to(new_domain_id)
    }
}
