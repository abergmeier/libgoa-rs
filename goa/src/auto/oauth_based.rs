// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_,pin::Pin};

glib::wrapper! {
    #[doc(alias = "GoaOAuthBased")]
    pub struct OAuthBased(Interface<ffi::GoaOAuthBased, ffi::GoaOAuthBasedIface>);

    match fn {
        type_ => || ffi::goa_oauth_based_get_type(),
    }
}

impl OAuthBased {
        pub const NONE: Option<&'static OAuthBased> = None;
    

    //#[doc(alias = "goa_oauth_based_interface_info")]
    //pub fn interface_info() -> /*Ignored*/Option<gio::DBusInterfaceInfo> {
    //    unsafe { TODO: call ffi:goa_oauth_based_interface_info() }
    //}

    //#[doc(alias = "goa_oauth_based_override_properties")]
    //pub fn override_properties(klass: /*Ignored*/&mut glib::ObjectClass, property_id_begin: u32) -> u32 {
    //    unsafe { TODO: call ffi:goa_oauth_based_override_properties() }
    //}
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::OAuthBased>> Sealed for T {}
}

pub trait OAuthBasedExt: IsA<OAuthBased> + sealed::Sealed + 'static {
    #[doc(alias = "goa_oauth_based_call_get_access_token")]
    fn call_get_access_token<P: FnOnce(Result<(glib::GString, glib::GString, i32), glib::Error>) + 'static>(&self, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P) {
        
                let main_context = glib::MainContext::ref_thread_default();
                let is_main_context_owner = main_context.is_owner();
                let has_acquired_main_context = (!is_main_context_owner)
                    .then(|| main_context.acquire().ok())
                    .flatten();
                assert!(
                    is_main_context_owner || has_acquired_main_context.is_some(),
                    "Async operations only allowed if the thread is owning the MainContext"
                );
        
        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> = Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn call_get_access_token_trampoline<P: FnOnce(Result<(glib::GString, glib::GString, i32), glib::Error>) + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = std::ptr::null_mut();
            let mut out_access_token = std::ptr::null_mut();
            let mut out_access_token_secret = std::ptr::null_mut();
            let mut out_expires_in = std::mem::MaybeUninit::uninit();
            let _ = ffi::goa_oauth_based_call_get_access_token_finish(_source_object as *mut _, &mut out_access_token, &mut out_access_token_secret, out_expires_in.as_mut_ptr(), res, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(out_access_token), from_glib_full(out_access_token_secret), out_expires_in.assume_init())) } else { Err(from_glib_full(error)) };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> = Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = call_get_access_token_trampoline::<P>;
        unsafe {
            ffi::goa_oauth_based_call_get_access_token(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    fn call_get_access_token_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<(glib::GString, glib::GString, i32), glib::Error>> + 'static>> {

        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.call_get_access_token(
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "goa_oauth_based_call_get_access_token_sync")]
    fn call_get_access_token_sync(&self, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<(glib::GString, glib::GString, i32), glib::Error> {
        unsafe {
            let mut out_access_token = std::ptr::null_mut();
            let mut out_access_token_secret = std::ptr::null_mut();
            let mut out_expires_in = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::goa_oauth_based_call_get_access_token_sync(self.as_ref().to_glib_none().0, &mut out_access_token, &mut out_access_token_secret, out_expires_in.as_mut_ptr(), cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok((from_glib_full(out_access_token), from_glib_full(out_access_token_secret), out_expires_in.assume_init())) } else { Err(from_glib_full(error)) }
        }
    }

    //#[doc(alias = "goa_oauth_based_complete_get_access_token")]
    //fn complete_get_access_token(&self, invocation: /*Ignored*/gio::DBusMethodInvocation, access_token: &str, access_token_secret: &str, expires_in: i32) {
    //    unsafe { TODO: call ffi:goa_oauth_based_complete_get_access_token() }
    //}

    #[doc(alias = "goa_oauth_based_dup_consumer_key")]
    fn dup_consumer_key(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_oauth_based_dup_consumer_key(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_oauth_based_dup_consumer_secret")]
    fn dup_consumer_secret(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_oauth_based_dup_consumer_secret(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_oauth_based_get_consumer_key")]
    #[doc(alias = "get_consumer_key")]
    fn consumer_key(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_oauth_based_get_consumer_key(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_oauth_based_get_consumer_secret")]
    #[doc(alias = "get_consumer_secret")]
    fn consumer_secret(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_oauth_based_get_consumer_secret(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_oauth_based_set_consumer_key")]
    fn set_consumer_key(&self, value: &str) {
        unsafe {
            ffi::goa_oauth_based_set_consumer_key(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "goa_oauth_based_set_consumer_secret")]
    fn set_consumer_secret(&self, value: &str) {
        unsafe {
            ffi::goa_oauth_based_set_consumer_secret(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    //#[doc(alias = "handle-get-access-token")]
    //fn connect_handle_get_access_token<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored invocation: Gio.DBusMethodInvocation
    //}

    #[doc(alias = "consumer-key")]
    fn connect_consumer_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_consumer_key_trampoline<P: IsA<OAuthBased>, F: Fn(&P) + 'static>(this: *mut ffi::GoaOAuthBased, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(OAuthBased::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::consumer-key\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_consumer_key_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "consumer-secret")]
    fn connect_consumer_secret_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_consumer_secret_trampoline<P: IsA<OAuthBased>, F: Fn(&P) + 'static>(this: *mut ffi::GoaOAuthBased, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(OAuthBased::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::consumer-secret\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_consumer_secret_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<OAuthBased>> OAuthBasedExt for O {}
