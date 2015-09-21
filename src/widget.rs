// exports from gphoto2-widget.h

use ::libc::{c_void,c_int,c_char,c_float};

use ::camera::Camera;
use ::context::GPContext;

#[repr(C)]
pub struct CameraWidget {
    widget_type: CameraWidgetType,
    label: [c_char; 256],
    info: [c_char; 1024],
    name: [c_char; 256],
    parent: *mut CameraWidget,
    value_string: *mut c_char,
    value_int: c_int,
    value_float: c_float,
    choice: *mut *mut c_char,
    choice_count: c_int,
    min: c_float,
    max: c_float,
    increment: c_float,
    children: *mut *mut CameraWidget,
    children_count: c_int,
    changed: c_int,
    readonly: c_int,
    ref_count: c_int,
    id: c_int,
    callback: CameraWidgetCallback,
}

pub type CameraWidgetCallback = extern "C" fn (camera: *mut Camera, widget: *mut CameraWidget, context: *mut GPContext) -> c_int;

#[repr(C)]
pub enum CameraWidgetType {
  GP_WIDGET_WINDOW  = 0,
  GP_WIDGET_SECTION = 1,
  GP_WIDGET_TEXT    = 2,
  GP_WIDGET_RANGE   = 3,
  GP_WIDGET_TOGGLE  = 4,
  GP_WIDGET_RADIO   = 5,
  GP_WIDGET_MENU    = 6,
  GP_WIDGET_BUTTON  = 7,
  GP_WIDGET_DATE    = 8,
}

extern "C" {
    pub fn gp_widget_new(widget_type: CameraWidgetType, label: *const c_char, widget: *mut *mut CameraWidget) -> c_int;
    pub fn gp_widget_free(widget: *mut CameraWidget) -> c_int;
    pub fn gp_widget_ref(widget: *mut CameraWidget) -> c_int;
    pub fn gp_widget_unref(widget: *mut CameraWidget) -> c_int;
    pub fn gp_widget_append(widget: *mut CameraWidget, child: *mut CameraWidget) -> c_int;
    pub fn gp_widget_prepend(widget: *mut CameraWidget, child: *mut CameraWidget) -> c_int;
    pub fn gp_widget_count_children(widget: *mut CameraWidget) -> c_int;
    pub fn gp_widget_get_child(widget: *mut CameraWidget, child_number: c_int, child: *mut *mut CameraWidget) -> c_int;
    pub fn gp_widget_get_child_by_label(widget: *mut CameraWidget, label: *const c_char, child: *mut *mut CameraWidget) -> c_int;
    pub fn gp_widget_get_child_by_id(widget: *mut CameraWidget, id: c_int, child: *mut *mut CameraWidget) -> c_int;
    pub fn gp_widget_get_child_by_name(widget: *mut CameraWidget, name: *const c_char, child: *mut *mut CameraWidget) -> c_int;
    pub fn gp_widget_get_root(widget: *mut CameraWidget, root: *mut *mut CameraWidget) -> c_int;
    pub fn gp_widget_get_parent(widget: *mut CameraWidget, parent: *mut *mut CameraWidget) -> c_int;
    pub fn gp_widget_set_value(widget: *mut CameraWidget, value: *const c_void) -> c_int;
    pub fn gp_widget_get_value(widget: *mut CameraWidget, value: *mut c_void) -> c_int;
    pub fn gp_widget_set_name(widget: *mut CameraWidget, name: *const c_char) -> c_int;
    pub fn gp_widget_get_name(widget: *mut CameraWidget, name: *mut *const c_char) -> c_int;
    pub fn gp_widget_set_info(widget: *mut CameraWidget, info: *const c_char) -> c_int;
    pub fn gp_widget_get_info(widget: *mut CameraWidget, info: *mut *const c_char) -> c_int;
    pub fn gp_widget_get_id(widget: *mut CameraWidget, id: *mut c_int) -> c_int;
    pub fn gp_widget_get_type(widget: *mut CameraWidget, widget_type: *mut CameraWidgetType) -> c_int;
    pub fn gp_widget_get_label(widget: *mut CameraWidget, label: *mut *const c_char) -> c_int;
    pub fn gp_widget_set_range(range: *mut CameraWidget, low: c_float, high: c_float, increment: c_float) -> c_int;
    pub fn gp_widget_get_range(range: *mut CameraWidget, min: *mut c_float, max: *mut c_float, increment: *mut c_float) -> c_int;
    pub fn gp_widget_add_choice(widget: *mut CameraWidget, choice: *const c_char) -> c_int;
    pub fn gp_widget_count_choices(widget: *mut CameraWidget) -> c_int;
    pub fn gp_widget_get_choice(widget: *mut CameraWidget, choice_number: c_int, choice: *mut *const c_char) -> c_int;
    pub fn gp_widget_changed(widget: *mut CameraWidget) -> c_int;
    pub fn gp_widget_set_changed(widget: *mut CameraWidget, changed: c_int) -> c_int;
    pub fn gp_widget_set_readonly(widget: *mut CameraWidget, readonly: c_int) -> c_int;
    pub fn gp_widget_get_readonly(widget: *mut CameraWidget, readonly: *mut c_int) -> c_int;
}
