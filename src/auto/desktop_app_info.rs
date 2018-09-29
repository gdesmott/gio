// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use AppInfo;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use AppLaunchContext;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DesktopAppInfo(Object<ffi::GDesktopAppInfo, ffi::GDesktopAppInfoClass>): AppInfo;

    match fn {
        get_type => || ffi::g_desktop_app_info_get_type(),
    }
}

impl DesktopAppInfo {
    pub fn new(desktop_id: &str) -> DesktopAppInfo {
        unsafe {
            from_glib_full(ffi::g_desktop_app_info_new(desktop_id.to_glib_none().0))
        }
    }

    pub fn new_from_filename<P: AsRef<std::path::Path>>(filename: P) -> DesktopAppInfo {
        unsafe {
            from_glib_full(ffi::g_desktop_app_info_new_from_filename(filename.as_ref().to_glib_none().0))
        }
    }

    pub fn new_from_keyfile(key_file: &glib::KeyFile) -> DesktopAppInfo {
        unsafe {
            from_glib_full(ffi::g_desktop_app_info_new_from_keyfile(key_file.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    pub fn get_implementations(interface: &str) -> Vec<DesktopAppInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_desktop_app_info_get_implementations(interface.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v2_42", deprecated)]
    pub fn set_desktop_env(desktop_env: &str) {
        unsafe {
            ffi::g_desktop_app_info_set_desktop_env(desktop_env.to_glib_none().0);
        }
    }
}

pub trait DesktopAppInfoExt {
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn get_action_name(&self, action_name: &str) -> Option<String>;

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn get_boolean(&self, key: &str) -> bool;

    fn get_categories(&self) -> Option<String>;

    fn get_filename(&self) -> Option<std::path::PathBuf>;

    fn get_generic_name(&self) -> Option<String>;

    fn get_is_hidden(&self) -> bool;

    fn get_keywords(&self) -> Vec<String>;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn get_locale_string(&self, key: &str) -> Option<String>;

    fn get_nodisplay(&self) -> bool;

    fn get_show_in<'a, P: Into<Option<&'a str>>>(&self, desktop_env: P) -> bool;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_startup_wm_class(&self) -> Option<String>;

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn get_string(&self, key: &str) -> Option<String>;

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn has_key(&self, key: &str) -> bool;

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn launch_action<'a, P: Into<Option<&'a AppLaunchContext>>>(&self, action_name: &str, launch_context: P);

    //fn launch_uris_as_manager<'a, 'b, 'c, P: Into<Option<&'a AppLaunchContext>>, Q: Into<Option<&'b /*Ignored*/glib::SpawnChildSetupFunc>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>, S: Into<Option<&'c /*Unimplemented*/DesktopAppLaunchCallback>>, T: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, uris: &[&str], launch_context: P, spawn_flags: /*Ignored*/glib::SpawnFlags, user_setup: Q, user_setup_data: R, pid_callback: S, pid_callback_data: T) -> Result<(), Error>;

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn list_actions(&self) -> Vec<String>;
}

impl<O: IsA<DesktopAppInfo>> DesktopAppInfoExt for O {
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn get_action_name(&self, action_name: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_desktop_app_info_get_action_name(self.to_glib_none().0, action_name.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn get_boolean(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::g_desktop_app_info_get_boolean(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn get_categories(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_desktop_app_info_get_categories(self.to_glib_none().0))
        }
    }

    fn get_filename(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_desktop_app_info_get_filename(self.to_glib_none().0))
        }
    }

    fn get_generic_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_desktop_app_info_get_generic_name(self.to_glib_none().0))
        }
    }

    fn get_is_hidden(&self) -> bool {
        unsafe {
            from_glib(ffi::g_desktop_app_info_get_is_hidden(self.to_glib_none().0))
        }
    }

    fn get_keywords(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_desktop_app_info_get_keywords(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn get_locale_string(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_desktop_app_info_get_locale_string(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn get_nodisplay(&self) -> bool {
        unsafe {
            from_glib(ffi::g_desktop_app_info_get_nodisplay(self.to_glib_none().0))
        }
    }

    fn get_show_in<'a, P: Into<Option<&'a str>>>(&self, desktop_env: P) -> bool {
        let desktop_env = desktop_env.into();
        let desktop_env = desktop_env.to_glib_none();
        unsafe {
            from_glib(ffi::g_desktop_app_info_get_show_in(self.to_glib_none().0, desktop_env.0))
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_startup_wm_class(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_desktop_app_info_get_startup_wm_class(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn get_string(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_desktop_app_info_get_string(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn has_key(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::g_desktop_app_info_has_key(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn launch_action<'a, P: Into<Option<&'a AppLaunchContext>>>(&self, action_name: &str, launch_context: P) {
        let launch_context = launch_context.into();
        let launch_context = launch_context.to_glib_none();
        unsafe {
            ffi::g_desktop_app_info_launch_action(self.to_glib_none().0, action_name.to_glib_none().0, launch_context.0);
        }
    }

    //fn launch_uris_as_manager<'a, 'b, 'c, P: Into<Option<&'a AppLaunchContext>>, Q: Into<Option<&'b /*Ignored*/glib::SpawnChildSetupFunc>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>, S: Into<Option<&'c /*Unimplemented*/DesktopAppLaunchCallback>>, T: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, uris: &[&str], launch_context: P, spawn_flags: /*Ignored*/glib::SpawnFlags, user_setup: Q, user_setup_data: R, pid_callback: S, pid_callback_data: T) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::g_desktop_app_info_launch_uris_as_manager() }
    //}

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn list_actions(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_desktop_app_info_list_actions(self.to_glib_none().0))
        }
    }
}
