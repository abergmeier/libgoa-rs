// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use goa_sys;
use std::fmt;

glib_wrapper! {
    pub struct Documents(Interface<goa_sys::GoaDocuments>);

    match fn {
        get_type => || goa_sys::goa_documents_get_type(),
    }
}

impl Documents {
    //pub fn interface_info() -> /*Ignored*/Option<gio::DBusInterfaceInfo> {
    //    unsafe { TODO: call goa_sys:goa_documents_interface_info() }
    //}

    //pub fn override_properties(klass: /*Ignored*/&mut glib::ObjectClass, property_id_begin: u32) -> u32 {
    //    unsafe { TODO: call goa_sys:goa_documents_override_properties() }
    //}
}

pub const NONE_DOCUMENTS: Option<&Documents> = None;

pub trait DocumentsExt: 'static {}

impl<O: IsA<Documents>> DocumentsExt for O {}

impl fmt::Display for Documents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Documents")
    }
}
