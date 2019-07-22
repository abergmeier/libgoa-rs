// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use goa_sys;
use std::fmt;

glib_wrapper! {
    pub struct ObjectManagerClient(Object<goa_sys::GoaObjectManagerClient, goa_sys::GoaObjectManagerClientClass, ObjectManagerClientClass>);

    match fn {
        get_type => || goa_sys::goa_object_manager_client_get_type(),
    }
}

impl ObjectManagerClient {
    //pub fn new_for_bus_sync<P: IsA<gio::Cancellable>>(bus_type: /*Ignored*/gio::BusType, flags: /*Ignored*/gio::DBusObjectManagerClientFlags, name: &str, object_path: &str, cancellable: Option<&P>) -> Result<ObjectManagerClient, Error> {
    //    unsafe { TODO: call goa_sys:goa_object_manager_client_new_for_bus_sync() }
    //}

    //pub fn new_sync<P: IsA<gio::Cancellable>>(connection: /*Ignored*/&gio::DBusConnection, flags: /*Ignored*/gio::DBusObjectManagerClientFlags, name: Option<&str>, object_path: &str, cancellable: Option<&P>) -> Result<ObjectManagerClient, Error> {
    //    unsafe { TODO: call goa_sys:goa_object_manager_client_new_sync() }
    //}

    //pub fn get_proxy_type(manager: /*Ignored*/&gio::DBusObjectManagerClient, object_path: &str, interface_name: Option<&str>, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> glib::types::Type {
    //    unsafe { TODO: call goa_sys:goa_object_manager_client_get_proxy_type() }
    //}

    //pub fn new<P: IsA<gio::Cancellable>, Q: FnOnce(Result<ObjectManagerClient, Error>) + Send + 'static>(connection: /*Ignored*/&gio::DBusConnection, flags: /*Ignored*/gio::DBusObjectManagerClientFlags, name: Option<&str>, object_path: &str, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call goa_sys:goa_object_manager_client_new() }
    //}

    //#[cfg(feature = "futures")]
    //pub fn new_future(connection: /*Ignored*/&gio::DBusConnection, flags: /*Ignored*/gio::DBusObjectManagerClientFlags, name: Option<&str>, object_path: &str) -> Box_<dyn future::Future<Output = Result<ObjectManagerClient, Error>> + std::marker::Unpin> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let connection = connection.clone();
        //let name = name.map(ToOwned::to_owned);
        //let object_path = String::from(object_path);
        //GioFuture::new(&(), move |_obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    Self::new(
        //        &connection,
        //        flags,
        //        name.as_ref().map(::std::borrow::Borrow::borrow),
        //        &object_path,
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    //pub fn new_for_bus<P: IsA<gio::Cancellable>, Q: FnOnce(Result<ObjectManagerClient, Error>) + Send + 'static>(bus_type: /*Ignored*/gio::BusType, flags: /*Ignored*/gio::DBusObjectManagerClientFlags, name: &str, object_path: &str, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call goa_sys:goa_object_manager_client_new_for_bus() }
    //}

    //#[cfg(feature = "futures")]
    //pub fn new_for_bus_future(bus_type: /*Ignored*/gio::BusType, flags: /*Ignored*/gio::DBusObjectManagerClientFlags, name: &str, object_path: &str) -> Box_<dyn future::Future<Output = Result<ObjectManagerClient, Error>> + std::marker::Unpin> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let name = String::from(name);
        //let object_path = String::from(object_path);
        //GioFuture::new(&(), move |_obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    Self::new_for_bus(
        //        bus_type,
        //        flags,
        //        &name,
        //        &object_path,
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}
}

pub const NONE_OBJECT_MANAGER_CLIENT: Option<&ObjectManagerClient> = None;

impl fmt::Display for ObjectManagerClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ObjectManagerClient")
    }
}