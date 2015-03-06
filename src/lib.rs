// Copyright (C) 2013-2015  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA

#![crate_name = "glib-2_0-sys"]
#![crate_type = "lib"]

#![allow(missing_copy_implementations)]
#![allow(unstable_features)]

#![feature(libc)]

extern crate libc;

pub mod types;
pub mod list;

use types::{gboolean, gchar, gint, gpointer};

#[repr(C)]
pub struct GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *const gchar
}

#[repr(C)]
pub struct GMainContext;

#[repr(C)]
pub struct GMainLoop;

pub type GQuark = u32;

extern {
    pub fn g_free(mem: gpointer);
    pub fn g_error_copy(error: *const GError) -> *mut GError;
    pub fn g_error_free(error: *mut GError);
    pub fn g_error_new_literal(domain: GQuark, code: gint, message: *const gchar) -> *mut GError;
    pub fn g_main_context_new() -> *mut GMainContext;
    pub fn g_main_context_ref(context: *mut GMainContext) -> *mut GMainContext;
    pub fn g_main_context_unref(context: *mut GMainContext);
    pub fn g_main_context_default() -> *mut GMainContext;
    pub fn g_main_context_push_thread_default(context: *mut GMainContext);
    pub fn g_main_context_pop_thread_default(context: *mut GMainContext);
    pub fn g_main_loop_new(ctx: *mut GMainContext, is_running: gboolean) -> *mut GMainLoop;
    pub fn g_main_loop_ref(l: *mut GMainLoop) -> *mut GMainLoop;
    pub fn g_main_loop_unref(l: *mut GMainLoop);
    pub fn g_main_loop_get_context(l: *mut GMainLoop) -> *mut GMainContext; 
    pub fn g_main_loop_run(l: *mut GMainLoop);
    pub fn g_main_loop_quit(l: *mut GMainLoop);
    pub fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    pub fn g_quark_to_string(quark: GQuark) -> *const gchar;
    pub fn g_strdup(str: *const gchar) -> *mut gchar;
}
