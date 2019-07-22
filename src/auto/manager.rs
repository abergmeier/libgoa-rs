// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use goa_sys;
use std::fmt;

glib_wrapper! {
    pub struct Manager(Interface<goa_sys::GoaManager>);

    match fn {
        get_type => || goa_sys::goa_manager_get_type(),
    }
}

impl Manager {
    //pub fn interface_info() -> /*Ignored*/Option<gio::DBusInterfaceInfo> {
    //    unsafe { TODO: call goa_sys:goa_manager_interface_info() }
    //}

    //pub fn override_properties(klass: /*Ignored*/&mut glib::ObjectClass, property_id_begin: u32) -> u32 {
    //    unsafe { TODO: call goa_sys:goa_manager_override_properties() }
    //}
}

pub const NONE_MANAGER: Option<&Manager> = None;

pub trait ManagerExt: 'static {
    //fn call_add_account<P: IsA<gio::Cancellable>, Q: FnOnce(Result<GString, Error>) + Send + 'static>(&self, arg_provider: &str, arg_identity: &str, arg_presentation_identity: &str, arg_credentials: /*Ignored*/&glib::Variant, arg_details: /*Ignored*/&glib::Variant, cancellable: Option<&P>, callback: Q);

    //#[cfg(feature = "futures")]
    //fn call_add_account_future(&self, arg_provider: &str, arg_identity: &str, arg_presentation_identity: &str, arg_credentials: /*Ignored*/&glib::Variant, arg_details: /*Ignored*/&glib::Variant) -> Box_<dyn future::Future<Output = Result<GString, Error>> + std::marker::Unpin>;

    //fn call_add_account_sync<P: IsA<gio::Cancellable>>(&self, arg_provider: &str, arg_identity: &str, arg_presentation_identity: &str, arg_credentials: /*Ignored*/&glib::Variant, arg_details: /*Ignored*/&glib::Variant, cancellable: Option<&P>) -> Result<GString, Error>;

    //fn complete_add_account(&self, invocation: /*Ignored*/&gio::DBusMethodInvocation, account_object_path: &str);

    //fn connect_handle_add_account<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Manager>> ManagerExt for O {
    //fn call_add_account<P: IsA<gio::Cancellable>, Q: FnOnce(Result<GString, Error>) + Send + 'static>(&self, arg_provider: &str, arg_identity: &str, arg_presentation_identity: &str, arg_credentials: /*Ignored*/&glib::Variant, arg_details: /*Ignored*/&glib::Variant, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call goa_sys:goa_manager_call_add_account() }
    //}

    //#[cfg(feature = "futures")]
    //fn call_add_account_future(&self, arg_provider: &str, arg_identity: &str, arg_presentation_identity: &str, arg_credentials: /*Ignored*/&glib::Variant, arg_details: /*Ignored*/&glib::Variant) -> Box_<dyn future::Future<Output = Result<GString, Error>> + std::marker::Unpin> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let arg_provider = String::from(arg_provider);
        //let arg_identity = String::from(arg_identity);
        //let arg_presentation_identity = String::from(arg_presentation_identity);
        //let arg_credentials = arg_credentials.clone();
        //let arg_details = arg_details.clone();
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    obj.call_add_account(
        //        &arg_provider,
        //        &arg_identity,
        //        &arg_presentation_identity,
        //        &arg_credentials,
        //        &arg_details,
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    //fn call_add_account_sync<P: IsA<gio::Cancellable>>(&self, arg_provider: &str, arg_identity: &str, arg_presentation_identity: &str, arg_credentials: /*Ignored*/&glib::Variant, arg_details: /*Ignored*/&glib::Variant, cancellable: Option<&P>) -> Result<GString, Error> {
    //    unsafe { TODO: call goa_sys:goa_manager_call_add_account_sync() }
    //}

    //fn complete_add_account(&self, invocation: /*Ignored*/&gio::DBusMethodInvocation, account_object_path: &str) {
    //    unsafe { TODO: call goa_sys:goa_manager_complete_add_account() }
    //}

    //fn connect_handle_add_account<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored invocation: Gio.DBusMethodInvocation
    //    Ignored arg_credentials: GLib.Variant
    //    Ignored arg_details: GLib.Variant
    //}
}

impl fmt::Display for Manager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Manager")
    }
}
