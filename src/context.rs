// exports from gphoto2-context.h

use ::libc::{c_void,c_char,c_uint,c_float};

#[repr(C)]
pub struct GPContext {
    __private: c_void
}

#[repr(C)]
pub enum GPContextFeedback {
    GP_CONTEXT_FEEDBACK_OK     = 0,
    GP_CONTEXT_FEEDBACK_CANCEL = 1,
}

pub const GP_CONTEXT_FEEDBACK_OK:     GPContextFeedback = GPContextFeedback::GP_CONTEXT_FEEDBACK_OK;
pub const GP_CONTEXT_FEEDBACK_CANCEL: GPContextFeedback = GPContextFeedback::GP_CONTEXT_FEEDBACK_CANCEL;

pub type GPContextIdleFunc           = extern "C" fn (context: *mut GPContext, data: *mut c_void);
pub type GPContextErrorFunc          = extern "C" fn (context: *mut GPContext, text: *const c_char, data: *mut c_void);
pub type GPContextStatusFunc         = extern "C" fn (context: *mut GPContext, text: *const c_char, data: *mut c_void);
pub type GPContextMessageFunc        = extern "C" fn (context: *mut GPContext, text: *const c_char, data: *mut c_void);
pub type GPContextQuestionFunc       = extern "C" fn (context: *mut GPContext, text: *const c_char, data: *mut c_void) -> GPContextFeedback;
pub type GPContextCancelFunc         = extern "C" fn (context: *mut GPContext, data: *mut c_void) -> GPContextFeedback;
pub type GPContextProgressStartFunc  = extern "C" fn (context: *mut GPContext, target: c_float, text: *const c_char, data: *mut c_void) -> c_uint;
pub type GPContextProgressUpdateFunc = extern "C" fn (context: *mut GPContext, id: c_uint, current: c_float, data: *mut c_void);
pub type GPContextProgressStopFunc   = extern "C" fn (context: *mut GPContext, id: c_uint, data: *mut c_void);

extern "C" {
    pub fn gp_context_new() -> *mut GPContext;
    pub fn gp_context_ref(context: *mut GPContext);
    pub fn gp_context_unref(context: *mut GPContext);
    pub fn gp_context_set_idle_func(context: *mut GPContext, func: GPContextIdleFunc, data: *mut c_void);
    pub fn gp_context_set_progress_funcs(context: *mut GPContext, start_func: GPContextProgressStartFunc, update_func: GPContextProgressUpdateFunc, stop_func: GPContextProgressStopFunc, data: *mut c_void);
    pub fn gp_context_set_error_func(context: *mut GPContext, func: GPContextErrorFunc, data: *mut c_void);
    pub fn gp_context_set_status_func(context: *mut GPContext, fun: GPContextStatusFunc, data: *mut c_void);
    pub fn gp_context_set_question_func(context: *mut GPContext, func: GPContextQuestionFunc, data: *mut c_void);
    pub fn gp_context_set_cancel_func(context: *mut GPContext, func: GPContextCancelFunc, data: *mut c_void);
    pub fn gp_context_set_message_func(context: *mut GPContext, func: GPContextMessageFunc, data: *mut c_void);
    pub fn gp_context_idle(context: *mut GPContext);
    pub fn gp_context_error(context: *mut GPContext, format: *const c_char, ...);
    pub fn gp_context_status(context: *mut GPContext, format: *const c_char, ...);
    pub fn gp_context_message(context: *mut GPContext, format: *const c_char, ...);
    pub fn gp_context_question(context: *mut GPContext, format: *const c_char, ...) -> GPContextFeedback;
    pub fn gp_context_cancel(context: *mut GPContext) -> GPContextFeedback;
    pub fn gp_context_progress_start(context: *mut GPContext, target: c_float, format: *const c_char, ...) -> c_uint;
    pub fn gp_context_progress_update(context: *mut GPContext, id: c_uint, current: c_float);
    pub fn gp_context_progress_stop(context: *mut GPContext, id: c_uint);
}
