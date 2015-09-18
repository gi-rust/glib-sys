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

#![crate_name = "glib_2_0_sys"]
#![crate_type = "lib"]

#![allow(missing_copy_implementations)]

extern crate gtypes;
extern crate libc;

use gtypes::*;

#[repr(C)]
pub struct GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *const gchar
}

pub enum GMainContext { }

pub enum GMainLoop { }

#[repr(C)]
pub struct GSource {
    callback_data: gpointer
    // ...
}

pub type GQuark = u32;

#[repr(C)]
pub struct GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList
}

pub type GDestroyNotify = extern "C" fn (gpointer);
pub type GSourceFunc = extern "C" fn (gpointer) -> gboolean;

pub const G_PRIORITY_DEFAULT: gint = 0;
pub const G_PRIORITY_DEFAULT_IDLE: gint = 200;
pub const G_PRIORITY_HIGH: gint = -100;
pub const G_PRIORITY_HIGH_IDLE: gint = 100;
pub const G_PRIORITY_LOW: gint = 300;

extern {
    pub fn g_free(mem: gpointer);
    pub fn g_error_copy(error: *const GError) -> *mut GError;
    pub fn g_error_free(error: *mut GError);
    pub fn g_error_new_literal(domain: GQuark, code: gint, message: *const gchar) -> *mut GError;
    pub fn g_idle_source_new() -> *mut GSource;
    pub fn g_main_context_new() -> *mut GMainContext;
    pub fn g_main_context_ref(context: *mut GMainContext) -> *mut GMainContext;
    pub fn g_main_context_unref(context: *mut GMainContext);
    pub fn g_main_context_default() -> *mut GMainContext;
    pub fn g_main_context_invoke_full(context: *mut GMainContext, priority: gint, function: GSourceFunc, data: gpointer, notify: Option<GDestroyNotify>);
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
    pub fn g_source_ref(source: *mut GSource) -> *mut GSource;
    pub fn g_source_unref(source: *mut GSource);
    pub fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    pub fn g_source_destroy(source: *mut GSource);
    pub fn g_source_set_callback(source: *mut GSource, func: GSourceFunc, data: gpointer, notify: Option<GDestroyNotify>);
    pub fn g_source_set_priority(source: *mut GSource, priority: gint);
    pub fn g_strdup(str: *const gchar) -> *mut gchar;
    pub fn g_timeout_source_new(interval: guint) -> *mut GSource;
    pub fn g_timeout_source_new_seconds(interval: guint) -> *mut GSource;
}
