use std::ffi::CString;
use std::marker::PhantomData;

pub struct Class<T> {
    pd_class: *mut puredata_sys::_class,
    phantom: PhantomData<T>,
}

impl<T> Class<T> {
    pub fn register_new(
        name: CString,
        creator: extern "C" fn() -> *mut ::std::os::raw::c_void,
        destroyer: Option<extern "C" fn(&mut T)>,
    ) -> Self {
        unsafe {
            let destroyer = match destroyer {
                None => None,
                Some(d) => Some(std::mem::transmute::<
                    extern "C" fn(&mut T),
                    unsafe extern "C" fn(),
                >(d)),
            };
            Self {
                pd_class: puredata_sys::class_new(
                    puredata_sys::gensym(name.as_ptr()),
                    Some(creator),
                    destroyer,
                    std::mem::size_of::<T>(),
                    0,
                    0,
                ),
                phantom: PhantomData,
            }
        }
    }

    pub fn add_bang(&mut self, m: extern "C" fn(&mut T)) {
        unsafe {
            puredata_sys::class_addbang(
                self.pd_class,
                Some(std::mem::transmute::<
                    extern "C" fn(&mut T),
                    unsafe extern "C" fn(),
                >(m)),
            );
        }
    }
}