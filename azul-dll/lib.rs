// WARNING: autogenerated code for azul api version 0.1.0
//! Public API for Azul
//!
//! A single function can have multiple implementations depending on whether it is
//! compiled for the Rust-desktop target, the Rust-wasm target or the C API.
//!
//! For now, the crate simply re-exports azul_core and calls the c_api functions

extern crate azul_core;
extern crate azul_css;
extern crate azul_native_style;
#[cfg(target_arch = "wasm32")]
extern crate azul_web;
#[cfg(not(target_arch = "wasm32"))]
extern crate azul_desktop;

use core::ffi::c_void;
use azul_core::dom::Dom;
use azul_core::callbacks::LayoutInfo;
use azul_css::Css;
use azul_core::window::WindowCreateOptions;
#[cfg(not(target_arch = "wasm32"))]
use azul_desktop::app::{App, AppConfig};
#[cfg(target_arch = "wasm32")]
use azul_web::app::{App, AppConfig};


/// The data model
pub type AzDataModel = *mut c_void;
/// The layout() callback fn
pub type AzLayoutCallback = fn(AzDataModel, AzLayoutInfoPtr) -> AzDomPtr;


/// Pointer to rust-allocated `Box<LayoutInfo>` struct
pub use ::azul_core::callbacks::LayoutInfoPtr as AzLayoutInfoPtr;
/// Destructor: Takes ownership of the `LayoutInfo` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_layout_info_delete<'a>(ptr: AzLayoutInfoPtr) { let _ = unsafe { Box::<LayoutInfo<'a>>::from_raw(ptr.ptr  as *mut LayoutInfo<'a>) }; }
/// (private): Downcasts the `AzLayoutInfoPtr` to a `Box<LayoutInfo<'a>>`. Note that this takes ownership of the pointer.
fn az_layout_info_downcast<'a>(ptr: AzLayoutInfoPtr) -> Box<LayoutInfo<'a>> { unsafe { Box::<LayoutInfo<'a>>::from_raw(ptr.ptr  as *mut LayoutInfo<'a>) } }

/// Pointer to rust-allocated `Box<AppConfig>` struct
#[no_mangle] #[repr(C)] pub struct AzAppConfigPtr { ptr: *mut c_void }
// Creates a new `AppConfig` instance whose memory is owned by the rust allocator
// Equivalent to the Rust `AppConfig::new()` constructor.
#[no_mangle] pub extern "C" fn az_app_config_new() -> AzAppConfigPtr { AzAppConfigPtr { ptr: Box::into_raw(Box::new(AppConfig::default())) as *mut c_void } }
/// Destructor: Takes ownership of the `AppConfig` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_app_config_delete(ptr: AzAppConfigPtr) { let _ = unsafe { Box::<AppConfig>::from_raw(ptr.ptr  as *mut AppConfig) }; }
/// (private): Downcasts the `AzAppConfigPtr` to a `Box<AppConfig>`. Note that this takes ownership of the pointer.
fn az_app_config_downcast(ptr: AzAppConfigPtr) -> Box<AppConfig> { unsafe { Box::<AppConfig>::from_raw(ptr.ptr  as *mut AppConfig) } }

/// Pointer to rust-allocated `Box<App>` struct
#[no_mangle] #[repr(C)] pub struct AzAppPtr { ptr: *mut c_void }
/// Creates a new App instance.
#[no_mangle] pub extern "C" fn az_app_new(data: AzDataModel, config: AzAppConfigPtr, callback: AzLayoutCallback) -> AzAppPtr { AzAppPtr { ptr: Box::into_raw(Box::new(App::new_with_callback(data, *az_app_config_downcast(config), callback))) as *mut c_void } }
// Equivalent to the Rust `App::run()` function.
#[no_mangle] pub extern "C" fn az_app_run(app: AzAppPtr, window: AzWindowCreateOptionsPtr) { az_app_downcast(app).run(*az_window_create_options_downcast(window)) }
/// Destructor: Takes ownership of the `App` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_app_delete(ptr: AzAppPtr) { let _ = unsafe { Box::<App<AzDataModel>>::from_raw(ptr.ptr  as *mut App<AzDataModel>) }; }
/// (private): Downcasts the `AzAppPtr` to a `Box<App<AzDataModel>>`. Note that this takes ownership of the pointer.
fn az_app_downcast(ptr: AzAppPtr) -> Box<App<AzDataModel>> { unsafe { Box::<App<AzDataModel>>::from_raw(ptr.ptr  as *mut App<AzDataModel>) } }

/// Pointer to rust-allocated `Box<WindowCreateOptions>` struct
#[no_mangle] #[repr(C)] pub struct AzWindowCreateOptionsPtr { ptr: *mut c_void }
// Creates a new `WindowCreateOptions` instance whose memory is owned by the rust allocator
// Equivalent to the Rust `WindowCreateOptions::new()` constructor.
#[no_mangle] pub extern "C" fn az_window_create_options_new(css: AzCssPtr) -> AzWindowCreateOptionsPtr { AzWindowCreateOptionsPtr { ptr: Box::into_raw(Box::new(WindowCreateOptions::<AzDataModel>::new(*az_css_downcast(css)))) as *mut c_void } }
/// Destructor: Takes ownership of the `WindowCreateOptions` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_window_create_options_delete(ptr: AzWindowCreateOptionsPtr) { let _ = unsafe { Box::<WindowCreateOptions<AzDataModel>>::from_raw(ptr.ptr  as *mut WindowCreateOptions<AzDataModel>) }; }
/// (private): Downcasts the `AzWindowCreateOptionsPtr` to a `Box<WindowCreateOptions<AzDataModel>>`. Note that this takes ownership of the pointer.
fn az_window_create_options_downcast(ptr: AzWindowCreateOptionsPtr) -> Box<WindowCreateOptions<AzDataModel>> { unsafe { Box::<WindowCreateOptions<AzDataModel>>::from_raw(ptr.ptr  as *mut WindowCreateOptions<AzDataModel>) } }

/// Pointer to rust-allocated `Box<Css>` struct
#[no_mangle] #[repr(C)] pub struct AzCssPtr { ptr: *mut c_void }
// Creates a new `Css` instance whose memory is owned by the rust allocator
// Equivalent to the Rust `Css::native()` constructor.
#[no_mangle] pub extern "C" fn az_css_native() -> AzCssPtr { AzCssPtr { ptr: Box::into_raw(Box::new(azul_native_style::native())) as *mut c_void } }
/// Destructor: Takes ownership of the `Css` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_css_delete(ptr: AzCssPtr) { let _ = unsafe { Box::<Css>::from_raw(ptr.ptr  as *mut Css) }; }
/// (private): Downcasts the `AzCssPtr` to a `Box<Css>`. Note that this takes ownership of the pointer.
fn az_css_downcast(ptr: AzCssPtr) -> Box<Css> { unsafe { Box::<Css>::from_raw(ptr.ptr  as *mut Css) } }

/// Pointer to rust-allocated `Box<Dom>` struct
pub use ::azul_core::dom::DomPtr as AzDomPtr;
// Creates a new `Dom` instance whose memory is owned by the rust allocator
// Equivalent to the Rust `Dom::div()` constructor.
#[no_mangle] pub extern "C" fn az_dom_div() -> AzDomPtr { AzDomPtr { ptr: Box::into_raw(Box::new(Dom::<AzDataModel>::div())) as *mut c_void } }
/// Destructor: Takes ownership of the `Dom` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_dom_delete(ptr: AzDomPtr) { let _ = unsafe { Box::<Dom<AzDataModel>>::from_raw(ptr.ptr  as *mut Dom<AzDataModel>) }; }
/// (private): Downcasts the `AzDomPtr` to a `Box<Dom<AzDataModel>>`. Note that this takes ownership of the pointer.
fn az_dom_downcast(ptr: AzDomPtr) -> Box<Dom<AzDataModel>> { unsafe { Box::<Dom<AzDataModel>>::from_raw(ptr.ptr  as *mut Dom<AzDataModel>) } }