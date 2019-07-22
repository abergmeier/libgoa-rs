// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::connect_raw;
use glib::translate::*;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib_sys;
use goa_sys;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct MediaServer(Interface<goa_sys::GoaMediaServer>);

    match fn {
        get_type => || goa_sys::goa_media_server_get_type(),
    }
}

impl MediaServer {
    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //pub fn interface_info() -> /*Ignored*/Option<gio::DBusInterfaceInfo> {
    //    unsafe { TODO: call goa_sys:goa_media_server_interface_info() }
    //}

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //pub fn override_properties(klass: /*Ignored*/&mut glib::ObjectClass, property_id_begin: u32) -> u32 {
    //    unsafe { TODO: call goa_sys:goa_media_server_override_properties() }
    //}
}

pub const NONE_MEDIA_SERVER: Option<&MediaServer> = None;

pub trait MediaServerExt: 'static {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn dup_udn(&self) -> Option<GString>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_dlna_supported(&self) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_udn(&self) -> Option<GString>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_dlna_supported(&self, value: bool);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_udn(&self, value: &str);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_dlna_supported_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_udn_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MediaServer>> MediaServerExt for O {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn dup_udn(&self) -> Option<GString> {
        unsafe {
            from_glib_full(goa_sys::goa_media_server_dup_udn(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_dlna_supported(&self) -> bool {
        unsafe {
            from_glib(goa_sys::goa_media_server_get_dlna_supported(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_udn(&self) -> Option<GString> {
        unsafe {
            from_glib_none(goa_sys::goa_media_server_get_udn(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_dlna_supported(&self, value: bool) {
        unsafe {
            goa_sys::goa_media_server_set_dlna_supported(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_udn(&self, value: &str) {
        unsafe {
            goa_sys::goa_media_server_set_udn(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_dlna_supported_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dlna_supported_trampoline<P, F: Fn(&P) + 'static>(this: *mut goa_sys::GoaMediaServer, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MediaServer>
        {
            let f: &F = &*(f as *const F);
            f(&MediaServer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::dlna-supported\0".as_ptr() as *const _,
                Some(transmute(notify_dlna_supported_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_udn_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_udn_trampoline<P, F: Fn(&P) + 'static>(this: *mut goa_sys::GoaMediaServer, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MediaServer>
        {
            let f: &F = &*(f as *const F);
            f(&MediaServer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::udn\0".as_ptr() as *const _,
                Some(transmute(notify_udn_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for MediaServer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MediaServer")
    }
}
