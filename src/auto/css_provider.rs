// This file was generated by gir (38add47) from gir-files (469db10)
// DO NOT EDIT

use CssSection;
use Error;
use StyleProvider;
use ffi;
use gio;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CssProvider(Object<ffi::GtkCssProvider, ffi::GtkCssProviderClass>): StyleProvider;

    match fn {
        get_type => || ffi::gtk_css_provider_get_type(),
    }
}

impl CssProvider {
    pub fn new() -> CssProvider {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_css_provider_new())
        }
    }

    pub fn get_default() -> Option<CssProvider> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_css_provider_get_default())
        }
    }

    pub fn get_named<'a, P: Into<Option<&'a str>>>(name: &str, variant: P) -> Option<CssProvider> {
        assert_initialized_main_thread!();
        let variant = variant.into();
        let variant = variant.to_glib_none();
        unsafe {
            from_glib_none(ffi::gtk_css_provider_get_named(name.to_glib_none().0, variant.0))
        }
    }
}

impl Default for CssProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CssProvider {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", CssProviderExt::to_string(self))
    }
}

pub trait CssProviderExt {
    fn load_from_data(&self, data: &[u8]) -> Result<(), Error>;

    fn load_from_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), Error>;

    fn load_from_path(&self, path: &str) -> Result<(), Error>;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn load_from_resource(&self, resource_path: &str);

    fn to_string(&self) -> String;

    fn connect_parsing_error<F: Fn(&Self, &CssSection, &Error) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CssProvider> + IsA<glib::object::Object>> CssProviderExt for O {
    fn load_from_data(&self, data: &[u8]) -> Result<(), Error> {
        let length = data.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_css_provider_load_from_data(self.to_glib_none().0, data.to_glib_none().0, length, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn load_from_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_css_provider_load_from_file(self.to_glib_none().0, file.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn load_from_path(&self, path: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_css_provider_load_from_path(self.to_glib_none().0, path.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn load_from_resource(&self, resource_path: &str) {
        unsafe {
            ffi::gtk_css_provider_load_from_resource(self.to_glib_none().0, resource_path.to_glib_none().0);
        }
    }

    fn to_string(&self) -> String {
        unsafe {
            from_glib_full(ffi::gtk_css_provider_to_string(self.to_glib_none().0))
        }
    }

    fn connect_parsing_error<F: Fn(&Self, &CssSection, &Error) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &CssSection, &Error) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "parsing-error",
                transmute(parsing_error_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn parsing_error_trampoline<P>(this: *mut ffi::GtkCssProvider, section: *mut ffi::GtkCssSection, error: *mut glib_ffi::GError, f: glib_ffi::gpointer)
where P: IsA<CssProvider> {
    callback_guard!();
    let f: &&(Fn(&P, &CssSection, &Error) + 'static) = transmute(f);
    f(&CssProvider::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(section), &from_glib_borrow(error))
}
