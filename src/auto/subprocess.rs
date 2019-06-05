// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use InputStream;
use OutputStream;
use SubprocessFlags;
#[cfg(feature = "futures")]
use futures::future;
use gio_sys;
use glib;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use std;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct Subprocess(Object<gio_sys::GSubprocess, SubprocessClass>);

    match fn {
        get_type => || gio_sys::g_subprocess_get_type(),
    }
}

impl Subprocess {
    //pub fn new(flags: SubprocessFlags, error: Option<&mut Error>, argv0: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Subprocess {
    //    unsafe { TODO: call gio_sys:g_subprocess_new() }
    //}

    pub fn newv(argv: &[&std::ffi::OsStr], flags: SubprocessFlags) -> Result<Subprocess, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_subprocess_newv(argv.to_glib_none().0, flags.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn communicate<P: IsA<Cancellable>>(&self, stdin_buf: Option<&glib::Bytes>, cancellable: Option<&P>) -> Result<(Option<glib::Bytes>, Option<glib::Bytes>), Error> {
        unsafe {
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_subprocess_communicate(self.to_glib_none().0, stdin_buf.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut stdout_buf, &mut stderr_buf, &mut error);
            if error.is_null() { Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf))) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn communicate_async<P: IsA<Cancellable>, Q: FnOnce(Result<(glib::Bytes, glib::Bytes), Error>) + Send + 'static>(&self, stdin_buf: Option<&glib::Bytes>, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn communicate_async_trampoline<Q: FnOnce(Result<(glib::Bytes, glib::Bytes), Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let _ = gio_sys::g_subprocess_communicate_finish(_source_object as *mut _, res, &mut stdout_buf, &mut stderr_buf, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf))) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = communicate_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_subprocess_communicate_async(self.to_glib_none().0, stdin_buf.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    pub fn communicate_async_future(&self, stdin_buf: Option<&glib::Bytes>) -> Box_<dyn future::Future<Output = Result<(glib::Bytes, glib::Bytes), Error>> + std::marker::Unpin> {
        use GioFuture;
        use fragile::Fragile;

        let stdin_buf = stdin_buf.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.communicate_async(
                stdin_buf.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    pub fn communicate_utf8<P: IsA<Cancellable>>(&self, stdin_buf: Option<&str>, cancellable: Option<&P>) -> Result<(Option<GString>, Option<GString>), Error> {
        unsafe {
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_subprocess_communicate_utf8(self.to_glib_none().0, stdin_buf.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut stdout_buf, &mut stderr_buf, &mut error);
            if error.is_null() { Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf))) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn force_exit(&self) {
        unsafe {
            gio_sys::g_subprocess_force_exit(self.to_glib_none().0);
        }
    }

    pub fn get_exit_status(&self) -> i32 {
        unsafe {
            gio_sys::g_subprocess_get_exit_status(self.to_glib_none().0)
        }
    }

    pub fn get_identifier(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_subprocess_get_identifier(self.to_glib_none().0))
        }
    }

    pub fn get_if_exited(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_subprocess_get_if_exited(self.to_glib_none().0))
        }
    }

    pub fn get_if_signaled(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_subprocess_get_if_signaled(self.to_glib_none().0))
        }
    }

    pub fn get_status(&self) -> i32 {
        unsafe {
            gio_sys::g_subprocess_get_status(self.to_glib_none().0)
        }
    }

    pub fn get_stderr_pipe(&self) -> Option<InputStream> {
        unsafe {
            from_glib_none(gio_sys::g_subprocess_get_stderr_pipe(self.to_glib_none().0))
        }
    }

    pub fn get_stdin_pipe(&self) -> Option<OutputStream> {
        unsafe {
            from_glib_none(gio_sys::g_subprocess_get_stdin_pipe(self.to_glib_none().0))
        }
    }

    pub fn get_stdout_pipe(&self) -> Option<InputStream> {
        unsafe {
            from_glib_none(gio_sys::g_subprocess_get_stdout_pipe(self.to_glib_none().0))
        }
    }

    pub fn get_successful(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_subprocess_get_successful(self.to_glib_none().0))
        }
    }

    pub fn get_term_sig(&self) -> i32 {
        unsafe {
            gio_sys::g_subprocess_get_term_sig(self.to_glib_none().0)
        }
    }

    #[cfg(any(not(windows), feature = "dox"))]
    pub fn send_signal(&self, signal_num: i32) {
        unsafe {
            gio_sys::g_subprocess_send_signal(self.to_glib_none().0, signal_num);
        }
    }

    pub fn wait<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_subprocess_wait(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn wait_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn wait_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_subprocess_wait_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = wait_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_subprocess_wait_async(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    pub fn wait_async_future(&self) -> Box_<dyn future::Future<Output = Result<(), Error>> + std::marker::Unpin> {
        use GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.wait_async(
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    pub fn wait_check<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_subprocess_wait_check(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn wait_check_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn wait_check_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_subprocess_wait_check_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = wait_check_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_subprocess_wait_check_async(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    pub fn wait_check_async_future(&self) -> Box_<dyn future::Future<Output = Result<(), Error>> + std::marker::Unpin> {
        use GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.wait_check_async(
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }
}

impl fmt::Display for Subprocess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Subprocess")
    }
}
