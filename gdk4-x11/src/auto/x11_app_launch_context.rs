// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use std::fmt;

glib::wrapper! {
    pub struct X11AppLaunchContext(Object<ffi::GdkX11AppLaunchContext, ffi::GdkX11AppLaunchContextClass>) @extends gdk::AppLaunchContext, gio::AppLaunchContext;

    match fn {
        type_ => || ffi::gdk_x11_app_launch_context_get_type(),
    }
}

impl X11AppLaunchContext {}

impl fmt::Display for X11AppLaunchContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("X11AppLaunchContext")
    }
}
