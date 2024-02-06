// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::{prelude::*};
#[cfg(feature = "v3_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
use glib::{signal::{connect_raw, SignalHandlerId},translate::*};
#[cfg(feature = "v3_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "GoaContacts")]
    pub struct Contacts(Interface<ffi::GoaContacts, ffi::GoaContactsIface>);

    match fn {
        type_ => || ffi::goa_contacts_get_type(),
    }
}

impl Contacts {
        pub const NONE: Option<&'static Contacts> = None;
    

    //#[doc(alias = "goa_contacts_interface_info")]
    //pub fn interface_info() -> /*Ignored*/Option<gio::DBusInterfaceInfo> {
    //    unsafe { TODO: call ffi:goa_contacts_interface_info() }
    //}

    //#[doc(alias = "goa_contacts_override_properties")]
    //pub fn override_properties(klass: /*Ignored*/&mut glib::ObjectClass, property_id_begin: u32) -> u32 {
    //    unsafe { TODO: call ffi:goa_contacts_override_properties() }
    //}
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Contacts>> Sealed for T {}
}

pub trait ContactsExt: IsA<Contacts> + sealed::Sealed + 'static {
    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_contacts_dup_uri")]
    fn dup_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_contacts_dup_uri(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_contacts_get_accept_ssl_errors")]
    #[doc(alias = "get_accept_ssl_errors")]
    fn accepts_ssl_errors(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_contacts_get_accept_ssl_errors(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_contacts_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_contacts_get_uri(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_contacts_set_accept_ssl_errors")]
    fn set_accept_ssl_errors(&self, value: bool) {
        unsafe {
            ffi::goa_contacts_set_accept_ssl_errors(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_contacts_set_uri")]
    fn set_uri(&self, value: &str) {
        unsafe {
            ffi::goa_contacts_set_uri(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "accept-ssl-errors")]
    fn connect_accept_ssl_errors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accept_ssl_errors_trampoline<P: IsA<Contacts>, F: Fn(&P) + 'static>(this: *mut ffi::GoaContacts, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Contacts::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accept-ssl-errors\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_accept_ssl_errors_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "uri")]
    fn connect_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uri_trampoline<P: IsA<Contacts>, F: Fn(&P) + 'static>(this: *mut ffi::GoaContacts, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Contacts::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::uri\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_uri_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Contacts>> ContactsExt for O {}
