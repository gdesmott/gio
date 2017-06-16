// This file was generated by gir (d121f7e) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v2_40")]
use Icon;
use NotificationPriority;
use ffi;
#[cfg(feature = "v2_40")]
use glib;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Notification(Object<ffi::GNotification>);

    match fn {
        get_type => || ffi::g_notification_get_type(),
    }
}

impl Notification {
    #[cfg(feature = "v2_40")]
    pub fn new(title: &str) -> Notification {
        unsafe {
            from_glib_full(ffi::g_notification_new(title.to_glib_none().0))
        }
    }
}

pub trait NotificationExt {
    #[cfg(feature = "v2_40")]
    fn add_button(&self, label: &str, detailed_action: &str);

    //#[cfg(feature = "v2_40")]
    //fn add_button_with_target<'a, P: Into<Option<&'a str>>>(&self, label: &str, action: &str, target_format: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg(feature = "v2_40")]
    fn add_button_with_target_value<'a, P: Into<Option<&'a glib::Variant>>>(&self, label: &str, action: &str, target: P);

    #[cfg(feature = "v2_40")]
    fn set_body<'a, P: Into<Option<&'a str>>>(&self, body: P);

    #[cfg(feature = "v2_40")]
    fn set_default_action(&self, detailed_action: &str);

    //#[cfg(feature = "v2_40")]
    //fn set_default_action_and_target<'a, P: Into<Option<&'a str>>>(&self, action: &str, target_format: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg(feature = "v2_40")]
    fn set_default_action_and_target_value<'a, P: Into<Option<&'a glib::Variant>>>(&self, action: &str, target: P);

    #[cfg(feature = "v2_40")]
    fn set_icon<P: IsA<Icon>>(&self, icon: &P);

    fn set_priority(&self, priority: NotificationPriority);

    #[cfg(feature = "v2_40")]
    fn set_title(&self, title: &str);

    #[cfg(feature = "v2_40")]
    fn set_urgent(&self, urgent: bool);
}

impl<O: IsA<Notification>> NotificationExt for O {
    #[cfg(feature = "v2_40")]
    fn add_button(&self, label: &str, detailed_action: &str) {
        unsafe {
            ffi::g_notification_add_button(self.to_glib_none().0, label.to_glib_none().0, detailed_action.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v2_40")]
    //fn add_button_with_target<'a, P: Into<Option<&'a str>>>(&self, label: &str, action: &str, target_format: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::g_notification_add_button_with_target() }
    //}

    #[cfg(feature = "v2_40")]
    fn add_button_with_target_value<'a, P: Into<Option<&'a glib::Variant>>>(&self, label: &str, action: &str, target: P) {
        let target = target.into();
        let target = target.to_glib_none();
        unsafe {
            ffi::g_notification_add_button_with_target_value(self.to_glib_none().0, label.to_glib_none().0, action.to_glib_none().0, target.0);
        }
    }

    #[cfg(feature = "v2_40")]
    fn set_body<'a, P: Into<Option<&'a str>>>(&self, body: P) {
        let body = body.into();
        let body = body.to_glib_none();
        unsafe {
            ffi::g_notification_set_body(self.to_glib_none().0, body.0);
        }
    }

    #[cfg(feature = "v2_40")]
    fn set_default_action(&self, detailed_action: &str) {
        unsafe {
            ffi::g_notification_set_default_action(self.to_glib_none().0, detailed_action.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v2_40")]
    //fn set_default_action_and_target<'a, P: Into<Option<&'a str>>>(&self, action: &str, target_format: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::g_notification_set_default_action_and_target() }
    //}

    #[cfg(feature = "v2_40")]
    fn set_default_action_and_target_value<'a, P: Into<Option<&'a glib::Variant>>>(&self, action: &str, target: P) {
        let target = target.into();
        let target = target.to_glib_none();
        unsafe {
            ffi::g_notification_set_default_action_and_target_value(self.to_glib_none().0, action.to_glib_none().0, target.0);
        }
    }

    #[cfg(feature = "v2_40")]
    fn set_icon<P: IsA<Icon>>(&self, icon: &P) {
        unsafe {
            ffi::g_notification_set_icon(self.to_glib_none().0, icon.to_glib_none().0);
        }
    }

    fn set_priority(&self, priority: NotificationPriority) {
        unsafe {
            ffi::g_notification_set_priority(self.to_glib_none().0, priority.to_glib());
        }
    }

    #[cfg(feature = "v2_40")]
    fn set_title(&self, title: &str) {
        unsafe {
            ffi::g_notification_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_40")]
    fn set_urgent(&self, urgent: bool) {
        unsafe {
            ffi::g_notification_set_urgent(self.to_glib_none().0, urgent.to_glib());
        }
    }
}
