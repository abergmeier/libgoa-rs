// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_,pin::Pin};

glib::wrapper! {
    #[doc(alias = "GoaAccount")]
    pub struct Account(Interface<ffi::GoaAccount, ffi::GoaAccountIface>);

    match fn {
        type_ => || ffi::goa_account_get_type(),
    }
}

impl Account {
        pub const NONE: Option<&'static Account> = None;
    

    //#[doc(alias = "goa_account_interface_info")]
    //pub fn interface_info() -> /*Ignored*/Option<gio::DBusInterfaceInfo> {
    //    unsafe { TODO: call ffi:goa_account_interface_info() }
    //}

    //#[doc(alias = "goa_account_override_properties")]
    //pub fn override_properties(klass: /*Ignored*/&mut glib::ObjectClass, property_id_begin: u32) -> u32 {
    //    unsafe { TODO: call ffi:goa_account_override_properties() }
    //}
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Account>> Sealed for T {}
}

pub trait AccountExt: IsA<Account> + sealed::Sealed + 'static {
    #[doc(alias = "goa_account_call_ensure_credentials")]
    fn call_ensure_credentials<P: FnOnce(Result<i32, glib::Error>) + 'static>(&self, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P) {
        
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
        unsafe extern "C" fn call_ensure_credentials_trampoline<P: FnOnce(Result<i32, glib::Error>) + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = std::ptr::null_mut();
            let mut out_expires_in = std::mem::MaybeUninit::uninit();
            let _ = ffi::goa_account_call_ensure_credentials_finish(_source_object as *mut _, out_expires_in.as_mut_ptr(), res, &mut error);
            let result = if error.is_null() { Ok(out_expires_in.assume_init()) } else { Err(from_glib_full(error)) };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> = Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = call_ensure_credentials_trampoline::<P>;
        unsafe {
            ffi::goa_account_call_ensure_credentials(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    fn call_ensure_credentials_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<i32, glib::Error>> + 'static>> {

        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.call_ensure_credentials(
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "goa_account_call_ensure_credentials_sync")]
    fn call_ensure_credentials_sync(&self, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<i32, glib::Error> {
        unsafe {
            let mut out_expires_in = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::goa_account_call_ensure_credentials_sync(self.as_ref().to_glib_none().0, out_expires_in.as_mut_ptr(), cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(out_expires_in.assume_init()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "goa_account_call_remove")]
    fn call_remove<P: FnOnce(Result<(), glib::Error>) + 'static>(&self, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P) {
        
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
        unsafe extern "C" fn call_remove_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = std::ptr::null_mut();
            let _ = ffi::goa_account_call_remove_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> = Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = call_remove_trampoline::<P>;
        unsafe {
            ffi::goa_account_call_remove(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    fn call_remove_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {

        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.call_remove(
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "goa_account_call_remove_sync")]
    fn call_remove_sync(&self, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::goa_account_call_remove_sync(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //#[doc(alias = "goa_account_complete_ensure_credentials")]
    //fn complete_ensure_credentials(&self, invocation: /*Ignored*/gio::DBusMethodInvocation, expires_in: i32) {
    //    unsafe { TODO: call ffi:goa_account_complete_ensure_credentials() }
    //}

    //#[doc(alias = "goa_account_complete_remove")]
    //fn complete_remove(&self, invocation: /*Ignored*/gio::DBusMethodInvocation) {
    //    unsafe { TODO: call ffi:goa_account_complete_remove() }
    //}

    #[doc(alias = "goa_account_dup_id")]
    fn dup_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_account_dup_id(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_dup_identity")]
    fn dup_identity(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_account_dup_identity(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_dup_presentation_identity")]
    fn dup_presentation_identity(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_account_dup_presentation_identity(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_dup_provider_icon")]
    fn dup_provider_icon(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_account_dup_provider_icon(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_dup_provider_name")]
    fn dup_provider_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_account_dup_provider_name(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_dup_provider_type")]
    fn dup_provider_type(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::goa_account_dup_provider_type(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_get_attention_needed")]
    #[doc(alias = "get_attention_needed")]
    fn is_attention_needed(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_attention_needed(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_get_calendar_disabled")]
    #[doc(alias = "get_calendar_disabled")]
    fn is_calendar_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_calendar_disabled(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_get_chat_disabled")]
    #[doc(alias = "get_chat_disabled")]
    fn is_chat_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_chat_disabled(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_get_contacts_disabled")]
    #[doc(alias = "get_contacts_disabled")]
    fn is_contacts_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_contacts_disabled(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_get_documents_disabled")]
    #[doc(alias = "get_documents_disabled")]
    fn is_documents_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_documents_disabled(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_account_get_files_disabled")]
    #[doc(alias = "get_files_disabled")]
    fn is_files_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_files_disabled(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_get_id")]
    #[doc(alias = "get_id")]
    fn id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_account_get_id(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_get_identity")]
    #[doc(alias = "get_identity")]
    fn identity(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_account_get_identity(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_16")))]
    #[doc(alias = "goa_account_get_is_locked")]
    #[doc(alias = "get_is_locked")]
    fn is_locked(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_is_locked(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_get_is_temporary")]
    #[doc(alias = "get_is_temporary")]
    fn is_temporary(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_is_temporary(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_get_mail_disabled")]
    #[doc(alias = "get_mail_disabled")]
    fn is_mail_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_mail_disabled(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_14")))]
    #[doc(alias = "goa_account_get_maps_disabled")]
    #[doc(alias = "get_maps_disabled")]
    fn is_maps_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_maps_disabled(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_18")))]
    #[doc(alias = "goa_account_get_music_disabled")]
    #[doc(alias = "get_music_disabled")]
    fn is_music_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_music_disabled(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_account_get_photos_disabled")]
    #[doc(alias = "get_photos_disabled")]
    fn is_photos_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_photos_disabled(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_get_presentation_identity")]
    #[doc(alias = "get_presentation_identity")]
    fn presentation_identity(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_account_get_presentation_identity(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "goa_account_get_printers_disabled")]
    #[doc(alias = "get_printers_disabled")]
    fn is_printers_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_printers_disabled(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_get_provider_icon")]
    #[doc(alias = "get_provider_icon")]
    fn provider_icon(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_account_get_provider_icon(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_get_provider_name")]
    #[doc(alias = "get_provider_name")]
    fn provider_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_account_get_provider_name(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_get_provider_type")]
    #[doc(alias = "get_provider_type")]
    fn provider_type(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::goa_account_get_provider_type(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "goa_account_get_read_later_disabled")]
    #[doc(alias = "get_read_later_disabled")]
    fn is_read_later_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_read_later_disabled(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_6")))]
    #[doc(alias = "goa_account_get_ticketing_disabled")]
    #[doc(alias = "get_ticketing_disabled")]
    fn is_ticketing_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_ticketing_disabled(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_26")))]
    #[doc(alias = "goa_account_get_todo_disabled")]
    #[doc(alias = "get_todo_disabled")]
    fn is_todo_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::goa_account_get_todo_disabled(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "goa_account_set_attention_needed")]
    fn set_attention_needed(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_attention_needed(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_account_set_calendar_disabled")]
    fn set_calendar_disabled(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_calendar_disabled(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_account_set_chat_disabled")]
    fn set_chat_disabled(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_chat_disabled(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_account_set_contacts_disabled")]
    fn set_contacts_disabled(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_contacts_disabled(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_account_set_documents_disabled")]
    fn set_documents_disabled(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_documents_disabled(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_account_set_files_disabled")]
    fn set_files_disabled(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_files_disabled(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_account_set_id")]
    fn set_id(&self, value: &str) {
        unsafe {
            ffi::goa_account_set_id(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "goa_account_set_identity")]
    fn set_identity(&self, value: &str) {
        unsafe {
            ffi::goa_account_set_identity(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_16")))]
    #[doc(alias = "goa_account_set_is_locked")]
    fn set_is_locked(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_is_locked(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_account_set_is_temporary")]
    fn set_is_temporary(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_is_temporary(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_account_set_mail_disabled")]
    fn set_mail_disabled(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_mail_disabled(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(feature = "v3_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_14")))]
    #[doc(alias = "goa_account_set_maps_disabled")]
    fn set_maps_disabled(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_maps_disabled(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(feature = "v3_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_18")))]
    #[doc(alias = "goa_account_set_music_disabled")]
    fn set_music_disabled(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_music_disabled(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "goa_account_set_photos_disabled")]
    fn set_photos_disabled(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_photos_disabled(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_account_set_presentation_identity")]
    fn set_presentation_identity(&self, value: &str) {
        unsafe {
            ffi::goa_account_set_presentation_identity(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "goa_account_set_printers_disabled")]
    fn set_printers_disabled(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_printers_disabled(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "goa_account_set_provider_icon")]
    fn set_provider_icon(&self, value: &str) {
        unsafe {
            ffi::goa_account_set_provider_icon(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "goa_account_set_provider_name")]
    fn set_provider_name(&self, value: &str) {
        unsafe {
            ffi::goa_account_set_provider_name(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "goa_account_set_provider_type")]
    fn set_provider_type(&self, value: &str) {
        unsafe {
            ffi::goa_account_set_provider_type(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "goa_account_set_read_later_disabled")]
    fn set_read_later_disabled(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_read_later_disabled(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(feature = "v3_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_6")))]
    #[doc(alias = "goa_account_set_ticketing_disabled")]
    fn set_ticketing_disabled(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_ticketing_disabled(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[cfg(feature = "v3_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_26")))]
    #[doc(alias = "goa_account_set_todo_disabled")]
    fn set_todo_disabled(&self, value: bool) {
        unsafe {
            ffi::goa_account_set_todo_disabled(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    //#[doc(alias = "handle-ensure-credentials")]
    //fn connect_handle_ensure_credentials<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored invocation: Gio.DBusMethodInvocation
    //}

    //#[doc(alias = "handle-remove")]
    //fn connect_handle_remove<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored invocation: Gio.DBusMethodInvocation
    //}

    #[doc(alias = "attention-needed")]
    fn connect_attention_needed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_attention_needed_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::attention-needed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_attention_needed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "calendar-disabled")]
    fn connect_calendar_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_calendar_disabled_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::calendar-disabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_calendar_disabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "chat-disabled")]
    fn connect_chat_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_chat_disabled_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::chat-disabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_chat_disabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "contacts-disabled")]
    fn connect_contacts_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_contacts_disabled_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::contacts-disabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_contacts_disabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "documents-disabled")]
    fn connect_documents_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_documents_disabled_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::documents-disabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_documents_disabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "files-disabled")]
    fn connect_files_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_files_disabled_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::files-disabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_files_disabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "id")]
    fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "identity")]
    fn connect_identity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_identity_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::identity\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_identity_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_16")))]
    #[doc(alias = "is-locked")]
    fn connect_is_locked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_locked_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::is-locked\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_is_locked_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "is-temporary")]
    fn connect_is_temporary_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_temporary_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::is-temporary\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_is_temporary_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "mail-disabled")]
    fn connect_mail_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mail_disabled_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::mail-disabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_mail_disabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_14")))]
    #[doc(alias = "maps-disabled")]
    fn connect_maps_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_maps_disabled_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::maps-disabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_maps_disabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_18")))]
    #[doc(alias = "music-disabled")]
    fn connect_music_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_music_disabled_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::music-disabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_music_disabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    #[doc(alias = "photos-disabled")]
    fn connect_photos_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_photos_disabled_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::photos-disabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_photos_disabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "presentation-identity")]
    fn connect_presentation_identity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_presentation_identity_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::presentation-identity\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_presentation_identity_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "printers-disabled")]
    fn connect_printers_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_printers_disabled_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::printers-disabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_printers_disabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "provider-icon")]
    fn connect_provider_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_provider_icon_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::provider-icon\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_provider_icon_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "provider-name")]
    fn connect_provider_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_provider_name_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::provider-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_provider_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "provider-type")]
    fn connect_provider_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_provider_type_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::provider-type\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_provider_type_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    #[doc(alias = "read-later-disabled")]
    fn connect_read_later_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_read_later_disabled_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::read-later-disabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_read_later_disabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_6")))]
    #[doc(alias = "ticketing-disabled")]
    fn connect_ticketing_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ticketing_disabled_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::ticketing-disabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_ticketing_disabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v3_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_26")))]
    #[doc(alias = "todo-disabled")]
    fn connect_todo_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_todo_disabled_trampoline<P: IsA<Account>, F: Fn(&P) + 'static>(this: *mut ffi::GoaAccount, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Account::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::todo-disabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_todo_disabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Account>> AccountExt for O {}
