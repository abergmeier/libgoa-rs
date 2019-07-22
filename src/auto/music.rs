// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use goa_sys;
use std::fmt;

glib_wrapper! {
    pub struct Music(Interface<goa_sys::GoaMusic>);

    match fn {
        get_type => || goa_sys::goa_music_get_type(),
    }
}

impl Music {
    //#[cfg(any(feature = "v3_18", feature = "dox"))]
    //pub fn interface_info() -> /*Ignored*/Option<gio::DBusInterfaceInfo> {
    //    unsafe { TODO: call goa_sys:goa_music_interface_info() }
    //}

    //#[cfg(any(feature = "v3_18", feature = "dox"))]
    //pub fn override_properties(klass: /*Ignored*/&mut glib::ObjectClass, property_id_begin: u32) -> u32 {
    //    unsafe { TODO: call goa_sys:goa_music_override_properties() }
    //}
}

pub const NONE_MUSIC: Option<&Music> = None;

pub trait MusicExt: 'static {}

impl<O: IsA<Music>> MusicExt for O {}

impl fmt::Display for Music {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Music")
    }
}