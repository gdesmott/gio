// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use libc;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Action;
use SettingsBackend;
use SettingsBindFlags;
use SettingsSchema;

glib_wrapper! {
    pub struct Settings(Object<gio_sys::GSettings, gio_sys::GSettingsClass, SettingsClass>);

    match fn {
        get_type => || gio_sys::g_settings_get_type(),
    }
}

impl Settings {
    pub fn new(schema_id: &str) -> Settings {
        unsafe { from_glib_full(gio_sys::g_settings_new(schema_id.to_glib_none().0)) }
    }

    pub fn new_full<P: IsA<SettingsBackend>>(
        schema: &SettingsSchema,
        backend: Option<&P>,
        path: Option<&str>,
    ) -> Settings {
        unsafe {
            from_glib_full(gio_sys::g_settings_new_full(
                schema.to_glib_none().0,
                backend.map(|p| p.as_ref()).to_glib_none().0,
                path.to_glib_none().0,
            ))
        }
    }

    pub fn new_with_backend<P: IsA<SettingsBackend>>(schema_id: &str, backend: &P) -> Settings {
        unsafe {
            from_glib_full(gio_sys::g_settings_new_with_backend(
                schema_id.to_glib_none().0,
                backend.as_ref().to_glib_none().0,
            ))
        }
    }

    pub fn new_with_backend_and_path<P: IsA<SettingsBackend>>(
        schema_id: &str,
        backend: &P,
        path: &str,
    ) -> Settings {
        unsafe {
            from_glib_full(gio_sys::g_settings_new_with_backend_and_path(
                schema_id.to_glib_none().0,
                backend.as_ref().to_glib_none().0,
                path.to_glib_none().0,
            ))
        }
    }

    pub fn new_with_path(schema_id: &str, path: &str) -> Settings {
        unsafe {
            from_glib_full(gio_sys::g_settings_new_with_path(
                schema_id.to_glib_none().0,
                path.to_glib_none().0,
            ))
        }
    }

    pub fn sync() {
        unsafe {
            gio_sys::g_settings_sync();
        }
    }

    pub fn unbind<P: IsA<glib::Object>>(object: &P, property: &str) {
        unsafe {
            gio_sys::g_settings_unbind(object.as_ref().to_glib_none().0, property.to_glib_none().0);
        }
    }
}

pub const NONE_SETTINGS: Option<&Settings> = None;

pub trait SettingsExt: 'static {
    fn apply(&self);

    fn bind<P: IsA<glib::Object>>(
        &self,
        key: &str,
        object: &P,
        property: &str,
        flags: SettingsBindFlags,
    );

    //fn bind_with_mapping<P: IsA<glib::Object>, Q: Fn(&glib::Value, &glib::Variant) -> bool + 'static, R: Fn(&glib::Value, &glib::VariantType) -> glib::Variant + 'static>(&self, key: &str, object: &P, property: &str, flags: SettingsBindFlags, get_mapping: Q, set_mapping: R);

    fn bind_writable<P: IsA<glib::Object>>(
        &self,
        key: &str,
        object: &P,
        property: &str,
        inverted: bool,
    );

    fn create_action(&self, key: &str) -> Option<Action>;

    fn delay(&self);

    //fn get(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_boolean(&self, key: &str) -> bool;

    fn get_child(&self, name: &str) -> Option<Settings>;

    fn get_default_value(&self, key: &str) -> Option<glib::Variant>;

    fn get_double(&self, key: &str) -> f64;

    fn get_enum(&self, key: &str) -> i32;

    fn get_flags(&self, key: &str) -> u32;

    fn get_has_unapplied(&self) -> bool;

    fn get_int(&self, key: &str) -> i32;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_int64(&self, key: &str) -> i64;

    //fn get_mapped(&self, key: &str, mapping: /*Unimplemented*/FnMut(&glib::Variant, /*Unimplemented*/Option<Fundamental: Pointer>) -> bool, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    fn get_string(&self, key: &str) -> Option<GString>;

    fn get_strv(&self, key: &str) -> Vec<GString>;

    fn get_uint(&self, key: &str) -> u32;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_uint64(&self, key: &str) -> u64;

    fn get_user_value(&self, key: &str) -> Option<glib::Variant>;

    fn get_value(&self, key: &str) -> Option<glib::Variant>;

    fn is_writable(&self, name: &str) -> bool;

    fn list_children(&self) -> Vec<GString>;

    fn list_keys(&self) -> Vec<GString>;

    fn reset(&self, key: &str);

    fn revert(&self);

    //fn set(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool;

    fn set_boolean(&self, key: &str, value: bool) -> bool;

    fn set_double(&self, key: &str, value: f64) -> bool;

    fn set_enum(&self, key: &str, value: i32) -> bool;

    fn set_flags(&self, key: &str, value: u32) -> bool;

    fn set_int(&self, key: &str, value: i32) -> bool;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn set_int64(&self, key: &str, value: i64) -> bool;

    fn set_string(&self, key: &str, value: &str) -> bool;

    fn set_strv(&self, key: &str, value: &[&str]) -> bool;

    fn set_uint(&self, key: &str, value: u32) -> bool;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn set_uint64(&self, key: &str, value: u64) -> bool;

    fn set_value(&self, key: &str, value: &glib::Variant) -> bool;

    fn get_property_backend(&self) -> Option<SettingsBackend>;

    fn get_property_delay_apply(&self) -> bool;

    fn get_property_path(&self) -> Option<GString>;

    fn get_property_schema_id(&self) -> Option<GString>;

    fn get_property_settings_schema(&self) -> Option<SettingsSchema>;

    //fn connect_change_event<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_writable_change_event<F: Fn(&Self, u32) -> Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_writable_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_delay_apply_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_unapplied_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Settings>> SettingsExt for O {
    fn apply(&self) {
        unsafe {
            gio_sys::g_settings_apply(self.as_ref().to_glib_none().0);
        }
    }

    fn bind<P: IsA<glib::Object>>(
        &self,
        key: &str,
        object: &P,
        property: &str,
        flags: SettingsBindFlags,
    ) {
        unsafe {
            gio_sys::g_settings_bind(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                object.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                flags.to_glib(),
            );
        }
    }

    //fn bind_with_mapping<P: IsA<glib::Object>, Q: Fn(&glib::Value, &glib::Variant) -> bool + 'static, R: Fn(&glib::Value, &glib::VariantType) -> glib::Variant + 'static>(&self, key: &str, object: &P, property: &str, flags: SettingsBindFlags, get_mapping: Q, set_mapping: R) {
    //    unsafe { TODO: call gio_sys:g_settings_bind_with_mapping() }
    //}

    fn bind_writable<P: IsA<glib::Object>>(
        &self,
        key: &str,
        object: &P,
        property: &str,
        inverted: bool,
    ) {
        unsafe {
            gio_sys::g_settings_bind_writable(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                object.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                inverted.to_glib(),
            );
        }
    }

    fn create_action(&self, key: &str) -> Option<Action> {
        unsafe {
            from_glib_full(gio_sys::g_settings_create_action(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn delay(&self) {
        unsafe {
            gio_sys::g_settings_delay(self.as_ref().to_glib_none().0);
        }
    }

    //fn get(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gio_sys:g_settings_get() }
    //}

    fn get_boolean(&self, key: &str) -> bool {
        unsafe {
            from_glib(gio_sys::g_settings_get_boolean(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn get_child(&self, name: &str) -> Option<Settings> {
        unsafe {
            from_glib_full(gio_sys::g_settings_get_child(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn get_default_value(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(gio_sys::g_settings_get_default_value(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn get_double(&self, key: &str) -> f64 {
        unsafe {
            gio_sys::g_settings_get_double(self.as_ref().to_glib_none().0, key.to_glib_none().0)
        }
    }

    fn get_enum(&self, key: &str) -> i32 {
        unsafe {
            gio_sys::g_settings_get_enum(self.as_ref().to_glib_none().0, key.to_glib_none().0)
        }
    }

    fn get_flags(&self, key: &str) -> u32 {
        unsafe {
            gio_sys::g_settings_get_flags(self.as_ref().to_glib_none().0, key.to_glib_none().0)
        }
    }

    fn get_has_unapplied(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_settings_get_has_unapplied(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_int(&self, key: &str) -> i32 {
        unsafe { gio_sys::g_settings_get_int(self.as_ref().to_glib_none().0, key.to_glib_none().0) }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_int64(&self, key: &str) -> i64 {
        unsafe {
            gio_sys::g_settings_get_int64(self.as_ref().to_glib_none().0, key.to_glib_none().0)
        }
    }

    //fn get_mapped(&self, key: &str, mapping: /*Unimplemented*/FnMut(&glib::Variant, /*Unimplemented*/Option<Fundamental: Pointer>) -> bool, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call gio_sys:g_settings_get_mapped() }
    //}

    fn get_string(&self, key: &str) -> Option<GString> {
        unsafe {
            from_glib_full(gio_sys::g_settings_get_string(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn get_strv(&self, key: &str) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gio_sys::g_settings_get_strv(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn get_uint(&self, key: &str) -> u32 {
        unsafe {
            gio_sys::g_settings_get_uint(self.as_ref().to_glib_none().0, key.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_uint64(&self, key: &str) -> u64 {
        unsafe {
            gio_sys::g_settings_get_uint64(self.as_ref().to_glib_none().0, key.to_glib_none().0)
        }
    }

    fn get_user_value(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(gio_sys::g_settings_get_user_value(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn get_value(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(gio_sys::g_settings_get_value(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn is_writable(&self, name: &str) -> bool {
        unsafe {
            from_glib(gio_sys::g_settings_is_writable(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn list_children(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gio_sys::g_settings_list_children(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn list_keys(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gio_sys::g_settings_list_keys(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reset(&self, key: &str) {
        unsafe {
            gio_sys::g_settings_reset(self.as_ref().to_glib_none().0, key.to_glib_none().0);
        }
    }

    fn revert(&self) {
        unsafe {
            gio_sys::g_settings_revert(self.as_ref().to_glib_none().0);
        }
    }

    //fn set(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call gio_sys:g_settings_set() }
    //}

    fn set_boolean(&self, key: &str, value: bool) -> bool {
        unsafe {
            from_glib(gio_sys::g_settings_set_boolean(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib(),
            ))
        }
    }

    fn set_double(&self, key: &str, value: f64) -> bool {
        unsafe {
            from_glib(gio_sys::g_settings_set_double(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                value,
            ))
        }
    }

    fn set_enum(&self, key: &str, value: i32) -> bool {
        unsafe {
            from_glib(gio_sys::g_settings_set_enum(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                value,
            ))
        }
    }

    fn set_flags(&self, key: &str, value: u32) -> bool {
        unsafe {
            from_glib(gio_sys::g_settings_set_flags(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                value,
            ))
        }
    }

    fn set_int(&self, key: &str, value: i32) -> bool {
        unsafe {
            from_glib(gio_sys::g_settings_set_int(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                value,
            ))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn set_int64(&self, key: &str, value: i64) -> bool {
        unsafe {
            from_glib(gio_sys::g_settings_set_int64(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                value,
            ))
        }
    }

    fn set_string(&self, key: &str, value: &str) -> bool {
        unsafe {
            from_glib(gio_sys::g_settings_set_string(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn set_strv(&self, key: &str, value: &[&str]) -> bool {
        unsafe {
            from_glib(gio_sys::g_settings_set_strv(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn set_uint(&self, key: &str, value: u32) -> bool {
        unsafe {
            from_glib(gio_sys::g_settings_set_uint(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                value,
            ))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn set_uint64(&self, key: &str, value: u64) -> bool {
        unsafe {
            from_glib(gio_sys::g_settings_set_uint64(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                value,
            ))
        }
    }

    fn set_value(&self, key: &str, value: &glib::Variant) -> bool {
        unsafe {
            from_glib(gio_sys::g_settings_set_value(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn get_property_backend(&self) -> Option<SettingsBackend> {
        unsafe {
            let mut value = Value::from_type(<SettingsBackend as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"backend\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `backend` getter")
        }
    }

    fn get_property_delay_apply(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"delay-apply\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `delay-apply` getter")
                .unwrap()
        }
    }

    fn get_property_path(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"path\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `path` getter")
        }
    }

    fn get_property_schema_id(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"schema-id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `schema-id` getter")
        }
    }

    fn get_property_settings_schema(&self) -> Option<SettingsSchema> {
        unsafe {
            let mut value = Value::from_type(<SettingsSchema as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"settings-schema\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `settings-schema` getter")
        }
    }

    //fn connect_change_event<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented keys: *.CArray TypeId { ns_id: 2, id: 4 }
    //}

    fn connect_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P, &str) + 'static>(
            this: *mut gio_sys::GSettings,
            key: *mut libc::c_char,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Settings>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Settings::from_glib_borrow(this).unsafe_cast(),
                &GString::from_glib_borrow(key),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute(changed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_writable_change_event<F: Fn(&Self, u32) -> Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn writable_change_event_trampoline<
            P,
            F: Fn(&P, u32) -> Inhibit + 'static,
        >(
            this: *mut gio_sys::GSettings,
            key: libc::c_uint,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<Settings>,
        {
            let f: &F = &*(f as *const F);
            f(&Settings::from_glib_borrow(this).unsafe_cast(), key).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"writable-change-event\0".as_ptr() as *const _,
                Some(transmute(
                    writable_change_event_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_writable_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn writable_changed_trampoline<P, F: Fn(&P, &str) + 'static>(
            this: *mut gio_sys::GSettings,
            key: *mut libc::c_char,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Settings>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Settings::from_glib_borrow(this).unsafe_cast(),
                &GString::from_glib_borrow(key),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"writable-changed\0".as_ptr() as *const _,
                Some(transmute(writable_changed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_delay_apply_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_delay_apply_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GSettings,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Settings>,
        {
            let f: &F = &*(f as *const F);
            f(&Settings::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::delay-apply\0".as_ptr() as *const _,
                Some(transmute(notify_delay_apply_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_has_unapplied_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_unapplied_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GSettings,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Settings>,
        {
            let f: &F = &*(f as *const F);
            f(&Settings::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-unapplied\0".as_ptr() as *const _,
                Some(transmute(
                    notify_has_unapplied_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Settings")
    }
}
