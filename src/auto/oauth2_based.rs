// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
#[cfg(feature = "futures")]
use futures::future;
use gio;
use gio_sys;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use goa_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct OAuth2Based(Interface<goa_sys::GoaOAuth2Based>);

    match fn {
        get_type => || goa_sys::goa_oauth2_based_get_type(),
    }
}

impl OAuth2Based {
    //pub fn interface_info() -> /*Ignored*/Option<gio::DBusInterfaceInfo> {
    //    unsafe { TODO: call goa_sys:goa_oauth2_based_interface_info() }
    //}

    //pub fn override_properties(klass: /*Ignored*/&mut glib::ObjectClass, property_id_begin: u32) -> u32 {
    //    unsafe { TODO: call goa_sys:goa_oauth2_based_override_properties() }
    //}
}

pub const NONE_OAUTH2_BASED: Option<&OAuth2Based> = None;

pub trait OAuth2BasedExt: 'static {
    fn call_get_access_token<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(GString, i32), Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn call_get_access_token_future(&self) -> Box_<dyn future::Future<Output = Result<(GString, i32), Error>> + std::marker::Unpin>;

    fn call_get_access_token_sync<P: IsA<gio::Cancellable>>(&self, cancellable: Option<&P>) -> Result<(GString, i32), Error>;

    //fn complete_get_access_token(&self, invocation: /*Ignored*/&gio::DBusMethodInvocation, access_token: &str, expires_in: i32);

    fn dup_client_id(&self) -> Option<GString>;

    fn dup_client_secret(&self) -> Option<GString>;

    fn get_client_id(&self) -> Option<GString>;

    fn get_client_secret(&self) -> Option<GString>;

    fn set_client_id(&self, value: &str);

    fn set_client_secret(&self, value: &str);

    //fn connect_handle_get_access_token<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_client_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_client_secret_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<OAuth2Based>> OAuth2BasedExt for O {
    fn call_get_access_token<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(GString, i32), Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn call_get_access_token_trampoline<Q: FnOnce(Result<(GString, i32), Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let mut out_access_token = ptr::null_mut();
            let mut out_expires_in = mem::MaybeUninit::uninit();
            let _ = goa_sys::goa_oauth2_based_call_get_access_token_finish(_source_object as *mut _, &mut out_access_token, out_expires_in.as_mut_ptr(), res, &mut error);
            let out_expires_in = out_expires_in.assume_init();
            let result = if error.is_null() { Ok((from_glib_full(out_access_token), out_expires_in)) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = call_get_access_token_trampoline::<Q>;
        unsafe {
            goa_sys::goa_oauth2_based_call_get_access_token(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn call_get_access_token_future(&self) -> Box_<dyn future::Future<Output = Result<(GString, i32), Error>> + std::marker::Unpin> {
        use gio::GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            obj.call_get_access_token(
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn call_get_access_token_sync<P: IsA<gio::Cancellable>>(&self, cancellable: Option<&P>) -> Result<(GString, i32), Error> {
        unsafe {
            let mut out_access_token = ptr::null_mut();
            let mut out_expires_in = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = goa_sys::goa_oauth2_based_call_get_access_token_sync(self.as_ref().to_glib_none().0, &mut out_access_token, out_expires_in.as_mut_ptr(), cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            let out_expires_in = out_expires_in.assume_init();
            if error.is_null() { Ok((from_glib_full(out_access_token), out_expires_in)) } else { Err(from_glib_full(error)) }
        }
    }

    //fn complete_get_access_token(&self, invocation: /*Ignored*/&gio::DBusMethodInvocation, access_token: &str, expires_in: i32) {
    //    unsafe { TODO: call goa_sys:goa_oauth2_based_complete_get_access_token() }
    //}

    fn dup_client_id(&self) -> Option<GString> {
        unsafe {
            from_glib_full(goa_sys::goa_oauth2_based_dup_client_id(self.as_ref().to_glib_none().0))
        }
    }

    fn dup_client_secret(&self) -> Option<GString> {
        unsafe {
            from_glib_full(goa_sys::goa_oauth2_based_dup_client_secret(self.as_ref().to_glib_none().0))
        }
    }

    fn get_client_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(goa_sys::goa_oauth2_based_get_client_id(self.as_ref().to_glib_none().0))
        }
    }

    fn get_client_secret(&self) -> Option<GString> {
        unsafe {
            from_glib_none(goa_sys::goa_oauth2_based_get_client_secret(self.as_ref().to_glib_none().0))
        }
    }

    fn set_client_id(&self, value: &str) {
        unsafe {
            goa_sys::goa_oauth2_based_set_client_id(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_client_secret(&self, value: &str) {
        unsafe {
            goa_sys::goa_oauth2_based_set_client_secret(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    //fn connect_handle_get_access_token<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored invocation: Gio.DBusMethodInvocation
    //}

    fn connect_property_client_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_client_id_trampoline<P, F: Fn(&P) + 'static>(this: *mut goa_sys::GoaOAuth2Based, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<OAuth2Based>
        {
            let f: &F = &*(f as *const F);
            f(&OAuth2Based::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::client-id\0".as_ptr() as *const _,
                Some(transmute(notify_client_id_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_client_secret_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_client_secret_trampoline<P, F: Fn(&P) + 'static>(this: *mut goa_sys::GoaOAuth2Based, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<OAuth2Based>
        {
            let f: &F = &*(f as *const F);
            f(&OAuth2Based::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::client-secret\0".as_ptr() as *const _,
                Some(transmute(notify_client_secret_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for OAuth2Based {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OAuth2Based")
    }
}