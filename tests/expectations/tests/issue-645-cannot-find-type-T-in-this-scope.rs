/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]

#[derive(Clone, Copy, Debug)] pub struct RefPtr<T>(T);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HasRefPtr<T> {
    pub refptr_member: RefPtr<HasRefPtr_TypedefOfT<T>>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
pub type HasRefPtr_TypedefOfT<T> = T;
impl <T> Default for HasRefPtr<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
