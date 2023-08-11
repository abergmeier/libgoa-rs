// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_,fmt,mem::transmute};

glib::wrapper! {
    #[doc(alias = "GoaMail")]
    pub struct Mail(Interface<ffi::GoaMail, ffi::GoaMailIface>);

    match fn {
        type_ => || ffi::goa_mail_get_type(),
    }
}

impl Mail {
        pub const NONE: Option<&'static Mail> = None;
    

    //#[doc(alias = "goa_mail_interface_info")]
    //pub fn interface_info() -> /*Ignored*/Option<gio::DBusInterfaceInfo> {
    //    unsafe { TODO: call ffi:goa_mail_interface_info() }
    //}

    //#[doc(alias = "goa_mail_override_properties")]
    //pub fn override_properties(klass: /*Ignored*/&mut glib::ObjectClass, property_id_begin: u32) -> u32 {
    //    unsafe { TODO: call ffi:goa_mail_override_properties() }
    //}
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Mail>> Sealed for T {}
}

pub trait MailExt: IsA<Mail> + sealed::Sealed + 'static {
    #[doc(alias = "goa_mail_dup_email_address")]
    fn dup_email_address(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_mail_dup_email_address(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_mail_dup_imap_host")]
    fn dup_imap_host(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_mail_dup_imap_host(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_mail_dup_imap_user_name")]
    fn dup_imap_user_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_mail_dup_imap_user_name(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_mail_dup_name")]
    fn dup_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_mail_dup_name(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_mail_dup_smtp_host")]
    fn dup_smtp_host(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_mail_dup_smtp_host(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_mail_dup_smtp_user_name")]
    fn dup_smtp_user_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_mail_dup_smtp_user_name(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_mail_get_email_address")]
    #[doc(alias = "get_email_address")]
    fn email_address(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_mail_get_email_address(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_mail_get_imap_accept_ssl_errors")]
    #[doc(alias = "get_imap_accept_ssl_errors")]
    fn is_imap_accept_ssl_errors(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_mail_get_imap_accept_ssl_errors(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_mail_get_imap_host")]
    #[doc(alias = "get_imap_host")]
    fn imap_host(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_mail_get_imap_host(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_mail_get_imap_supported")]
    #[doc(alias = "get_imap_supported")]
    fn is_imap_supported(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_mail_get_imap_supported(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_mail_get_imap_use_ssl")]
    #[doc(alias = "get_imap_use_ssl")]
    fn is_imap_use_ssl(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_mail_get_imap_use_ssl(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_mail_get_imap_use_tls")]
    #[doc(alias = "get_imap_use_tls")]
    fn is_imap_use_tls(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_mail_get_imap_use_tls(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_mail_get_imap_user_name")]
    #[doc(alias = "get_imap_user_name")]
    fn imap_user_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_mail_get_imap_user_name(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_mail_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_mail_get_name(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_mail_get_smtp_accept_ssl_errors")]
    #[doc(alias = "get_smtp_accept_ssl_errors")]
    fn is_smtp_accept_ssl_errors(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_mail_get_smtp_accept_ssl_errors(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "goa_mail_get_smtp_auth_login")]
    #[doc(alias = "get_smtp_auth_login")]
    fn is_smtp_auth_login(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_mail_get_smtp_auth_login(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "goa_mail_get_smtp_auth_plain")]
    #[doc(alias = "get_smtp_auth_plain")]
    fn is_smtp_auth_plain(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_mail_get_smtp_auth_plain(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "goa_mail_get_smtp_auth_xoauth2")]
    #[doc(alias = "get_smtp_auth_xoauth2")]
    fn is_smtp_auth_xoauth2(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_mail_get_smtp_auth_xoauth2(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_mail_get_smtp_host")]
    #[doc(alias = "get_smtp_host")]
    fn smtp_host(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_mail_get_smtp_host(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_mail_get_smtp_supported")]
    #[doc(alias = "get_smtp_supported")]
    fn is_smtp_supported(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_mail_get_smtp_supported(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_mail_get_smtp_use_auth")]
    #[doc(alias = "get_smtp_use_auth")]
    fn is_smtp_use_auth(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_mail_get_smtp_use_auth(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_mail_get_smtp_use_ssl")]
    #[doc(alias = "get_smtp_use_ssl")]
    fn is_smtp_use_ssl(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_mail_get_smtp_use_ssl(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_mail_get_smtp_use_tls")]
    #[doc(alias = "get_smtp_use_tls")]
    fn is_smtp_use_tls(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_mail_get_smtp_use_tls(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_mail_get_smtp_user_name")]
    #[doc(alias = "get_smtp_user_name")]
    fn smtp_user_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_mail_get_smtp_user_name(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_mail_set_email_address")]
    fn set_email_address(&self, value: &str) {
        unsafe {
            ffi::goa_mail_set_email_address(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_mail_set_imap_accept_ssl_errors")]
    fn set_imap_accept_ssl_errors(&self, value: bool) {
        unsafe {
            ffi::goa_mail_set_imap_accept_ssl_errors(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_mail_set_imap_host")]
    fn set_imap_host(&self, value: &str) {
        unsafe {
            ffi::goa_mail_set_imap_host(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "goa_mail_set_imap_supported")]
    fn set_imap_supported(&self, value: bool) {
        unsafe {
            ffi::goa_mail_set_imap_supported(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_mail_set_imap_use_ssl")]
    fn set_imap_use_ssl(&self, value: bool) {
        unsafe {
            ffi::goa_mail_set_imap_use_ssl(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_mail_set_imap_use_tls")]
    fn set_imap_use_tls(&self, value: bool) {
        unsafe {
            ffi::goa_mail_set_imap_use_tls(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_mail_set_imap_user_name")]
    fn set_imap_user_name(&self, value: &str) {
        unsafe {
            ffi::goa_mail_set_imap_user_name(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_mail_set_name")]
    fn set_name(&self, value: &str) {
        unsafe {
            ffi::goa_mail_set_name(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_mail_set_smtp_accept_ssl_errors")]
    fn set_smtp_accept_ssl_errors(&self, value: bool) {
        unsafe {
            ffi::goa_mail_set_smtp_accept_ssl_errors(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "goa_mail_set_smtp_auth_login")]
    fn set_smtp_auth_login(&self, value: bool) {
        unsafe {
            ffi::goa_mail_set_smtp_auth_login(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "goa_mail_set_smtp_auth_plain")]
    fn set_smtp_auth_plain(&self, value: bool) {
        unsafe {
            ffi::goa_mail_set_smtp_auth_plain(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "goa_mail_set_smtp_auth_xoauth2")]
    fn set_smtp_auth_xoauth2(&self, value: bool) {
        unsafe {
            ffi::goa_mail_set_smtp_auth_xoauth2(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_mail_set_smtp_host")]
    fn set_smtp_host(&self, value: &str) {
        unsafe {
            ffi::goa_mail_set_smtp_host(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "goa_mail_set_smtp_supported")]
    fn set_smtp_supported(&self, value: bool) {
        unsafe {
            ffi::goa_mail_set_smtp_supported(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_mail_set_smtp_use_auth")]
    fn set_smtp_use_auth(&self, value: bool) {
        unsafe {
            ffi::goa_mail_set_smtp_use_auth(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_mail_set_smtp_use_ssl")]
    fn set_smtp_use_ssl(&self, value: bool) {
        unsafe {
            ffi::goa_mail_set_smtp_use_ssl(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_mail_set_smtp_use_tls")]
    fn set_smtp_use_tls(&self, value: bool) {
        unsafe {
            ffi::goa_mail_set_smtp_use_tls(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_mail_set_smtp_user_name")]
    fn set_smtp_user_name(&self, value: &str) {
        unsafe {
            ffi::goa_mail_set_smtp_user_name(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "email-address")]
    fn connect_email_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_email_address_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::email-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_email_address_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "imap-accept-ssl-errors")]
    fn connect_imap_accept_ssl_errors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_imap_accept_ssl_errors_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::imap-accept-ssl-errors\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_imap_accept_ssl_errors_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "imap-host")]
    fn connect_imap_host_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_imap_host_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::imap-host\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_imap_host_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "imap-supported")]
    fn connect_imap_supported_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_imap_supported_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::imap-supported\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_imap_supported_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "imap-use-ssl")]
    fn connect_imap_use_ssl_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_imap_use_ssl_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::imap-use-ssl\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_imap_use_ssl_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "imap-use-tls")]
    fn connect_imap_use_tls_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_imap_use_tls_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::imap-use-tls\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_imap_use_tls_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "imap-user-name")]
    fn connect_imap_user_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_imap_user_name_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::imap-user-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_imap_user_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "smtp-accept-ssl-errors")]
    fn connect_smtp_accept_ssl_errors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_smtp_accept_ssl_errors_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::smtp-accept-ssl-errors\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_smtp_accept_ssl_errors_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "smtp-auth-login")]
    fn connect_smtp_auth_login_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_smtp_auth_login_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::smtp-auth-login\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_smtp_auth_login_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "smtp-auth-plain")]
    fn connect_smtp_auth_plain_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_smtp_auth_plain_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::smtp-auth-plain\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_smtp_auth_plain_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "smtp-auth-xoauth2")]
    fn connect_smtp_auth_xoauth2_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_smtp_auth_xoauth2_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::smtp-auth-xoauth2\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_smtp_auth_xoauth2_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "smtp-host")]
    fn connect_smtp_host_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_smtp_host_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::smtp-host\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_smtp_host_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "smtp-supported")]
    fn connect_smtp_supported_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_smtp_supported_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::smtp-supported\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_smtp_supported_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "smtp-use-auth")]
    fn connect_smtp_use_auth_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_smtp_use_auth_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::smtp-use-auth\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_smtp_use_auth_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "smtp-use-ssl")]
    fn connect_smtp_use_ssl_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_smtp_use_ssl_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::smtp-use-ssl\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_smtp_use_ssl_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "smtp-use-tls")]
    fn connect_smtp_use_tls_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_smtp_use_tls_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::smtp-use-tls\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_smtp_use_tls_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "smtp-user-name")]
    fn connect_smtp_user_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_smtp_user_name_trampoline<P: IsA<Mail>, F: Fn(&P) + 'static>(this: *mut ffi::GoaMail, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mail::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::smtp-user-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_smtp_user_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Mail>> MailExt for O {}

impl fmt::Display for Mail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Mail")
    }
}
