// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::{prelude::*};
use std::{fmt};

glib::wrapper! {
    #[doc(alias = "GoaTodo")]
    pub struct Todo(Interface<ffi::GoaTodo, ffi::GoaTodoIface>);

    match fn {
        type_ => || ffi::goa_todo_get_type(),
    }
}

impl Todo {
        pub const NONE: Option<&'static Todo> = None;
    

    //#[doc(alias = "goa_todo_interface_info")]
    //pub fn interface_info() -> /*Ignored*/Option<gio::DBusInterfaceInfo> {
    //    unsafe { TODO: call ffi:goa_todo_interface_info() }
    //}

    //#[doc(alias = "goa_todo_override_properties")]
    //pub fn override_properties(klass: /*Ignored*/&mut glib::ObjectClass, property_id_begin: u32) -> u32 {
    //    unsafe { TODO: call ffi:goa_todo_override_properties() }
    //}
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Todo>> Sealed for T {}
}

pub trait TodoExt: IsA<Todo> + sealed::Sealed + 'static {}

impl<O: IsA<Todo>> TodoExt for O {}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Todo")
    }
}
