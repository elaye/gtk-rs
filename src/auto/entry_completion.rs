// This file was generated by gir (38add47) from gir-files (469db10)
// DO NOT EDIT

use Buildable;
use CellArea;
use CellLayout;
use TreeIter;
use TreeModel;
use Widget;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct EntryCompletion(Object<ffi::GtkEntryCompletion, ffi::GtkEntryCompletionClass>): Buildable, CellLayout;

    match fn {
        get_type => || ffi::gtk_entry_completion_get_type(),
    }
}

impl EntryCompletion {
    pub fn new() -> EntryCompletion {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_entry_completion_new())
        }
    }

    pub fn new_with_area<P: IsA<CellArea>>(area: &P) -> EntryCompletion {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_entry_completion_new_with_area(area.to_glib_none().0))
        }
    }
}

impl Default for EntryCompletion {
    fn default() -> Self {
        Self::new()
    }
}

pub trait EntryCompletionExt {
    fn complete(&self);

    fn compute_prefix(&self, key: &str) -> Option<String>;

    fn delete_action(&self, index_: i32);

    fn get_completion_prefix(&self) -> Option<String>;

    fn get_entry(&self) -> Option<Widget>;

    fn get_inline_completion(&self) -> bool;

    fn get_inline_selection(&self) -> bool;

    fn get_minimum_key_length(&self) -> i32;

    fn get_model(&self) -> Option<TreeModel>;

    fn get_popup_completion(&self) -> bool;

    fn get_popup_set_width(&self) -> bool;

    fn get_popup_single_match(&self) -> bool;

    fn get_text_column(&self) -> i32;

    fn insert_action_markup(&self, index_: i32, markup: &str);

    fn insert_action_text(&self, index_: i32, text: &str);

    fn insert_prefix(&self);

    fn set_inline_completion(&self, inline_completion: bool);

    fn set_inline_selection(&self, inline_selection: bool);

    //fn set_match_func<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/EntryCompletionMatchFunc, func_data: P, func_notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn set_minimum_key_length(&self, length: i32);

    fn set_model<'a, P: IsA<TreeModel> + 'a, Q: Into<Option<&'a P>>>(&self, model: Q);

    fn set_popup_completion(&self, popup_completion: bool);

    fn set_popup_set_width(&self, popup_set_width: bool);

    fn set_popup_single_match(&self, popup_single_match: bool);

    fn set_text_column(&self, column: i32);

    fn get_property_cell_area(&self) -> Option<CellArea>;

    fn connect_action_activated<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_cursor_on_match<F: Fn(&Self, &TreeModel, &TreeIter) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_insert_prefix<F: Fn(&Self, &str) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_match_selected<F: Fn(&Self, &TreeModel, &TreeIter) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_no_matches<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cell_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inline_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inline_selection_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_minimum_key_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popup_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popup_set_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popup_single_match_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<EntryCompletion> + IsA<glib::object::Object>> EntryCompletionExt for O {
    fn complete(&self) {
        unsafe {
            ffi::gtk_entry_completion_complete(self.to_glib_none().0);
        }
    }

    fn compute_prefix(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_entry_completion_compute_prefix(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn delete_action(&self, index_: i32) {
        unsafe {
            ffi::gtk_entry_completion_delete_action(self.to_glib_none().0, index_);
        }
    }

    fn get_completion_prefix(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_completion_get_completion_prefix(self.to_glib_none().0))
        }
    }

    fn get_entry(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_entry_completion_get_entry(self.to_glib_none().0))
        }
    }

    fn get_inline_completion(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_inline_completion(self.to_glib_none().0))
        }
    }

    fn get_inline_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_inline_selection(self.to_glib_none().0))
        }
    }

    fn get_minimum_key_length(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_completion_get_minimum_key_length(self.to_glib_none().0)
        }
    }

    fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_entry_completion_get_model(self.to_glib_none().0))
        }
    }

    fn get_popup_completion(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_completion(self.to_glib_none().0))
        }
    }

    fn get_popup_set_width(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_set_width(self.to_glib_none().0))
        }
    }

    fn get_popup_single_match(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_single_match(self.to_glib_none().0))
        }
    }

    fn get_text_column(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_completion_get_text_column(self.to_glib_none().0)
        }
    }

    fn insert_action_markup(&self, index_: i32, markup: &str) {
        unsafe {
            ffi::gtk_entry_completion_insert_action_markup(self.to_glib_none().0, index_, markup.to_glib_none().0);
        }
    }

    fn insert_action_text(&self, index_: i32, text: &str) {
        unsafe {
            ffi::gtk_entry_completion_insert_action_text(self.to_glib_none().0, index_, text.to_glib_none().0);
        }
    }

    fn insert_prefix(&self) {
        unsafe {
            ffi::gtk_entry_completion_insert_prefix(self.to_glib_none().0);
        }
    }

    fn set_inline_completion(&self, inline_completion: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_inline_completion(self.to_glib_none().0, inline_completion.to_glib());
        }
    }

    fn set_inline_selection(&self, inline_selection: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_inline_selection(self.to_glib_none().0, inline_selection.to_glib());
        }
    }

    //fn set_match_func<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/EntryCompletionMatchFunc, func_data: P, func_notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_entry_completion_set_match_func() }
    //}

    fn set_minimum_key_length(&self, length: i32) {
        unsafe {
            ffi::gtk_entry_completion_set_minimum_key_length(self.to_glib_none().0, length);
        }
    }

    fn set_model<'a, P: IsA<TreeModel> + 'a, Q: Into<Option<&'a P>>>(&self, model: Q) {
        let model = model.into();
        let model = model.to_glib_none();
        unsafe {
            ffi::gtk_entry_completion_set_model(self.to_glib_none().0, model.0);
        }
    }

    fn set_popup_completion(&self, popup_completion: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_completion(self.to_glib_none().0, popup_completion.to_glib());
        }
    }

    fn set_popup_set_width(&self, popup_set_width: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_set_width(self.to_glib_none().0, popup_set_width.to_glib());
        }
    }

    fn set_popup_single_match(&self, popup_single_match: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_single_match(self.to_glib_none().0, popup_single_match.to_glib());
        }
    }

    fn set_text_column(&self, column: i32) {
        unsafe {
            ffi::gtk_entry_completion_set_text_column(self.to_glib_none().0, column);
        }
    }

    fn get_property_cell_area(&self) -> Option<CellArea> {
        unsafe {
            let mut value = Value::from_type(<CellArea as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "cell-area".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_action_activated<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "action-activated",
                transmute(action_activated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_cursor_on_match<F: Fn(&Self, &TreeModel, &TreeIter) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TreeModel, &TreeIter) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cursor-on-match",
                transmute(cursor_on_match_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_insert_prefix<F: Fn(&Self, &str) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "insert-prefix",
                transmute(insert_prefix_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_match_selected<F: Fn(&Self, &TreeModel, &TreeIter) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TreeModel, &TreeIter) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "match-selected",
                transmute(match_selected_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_no_matches<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "no-matches",
                transmute(no_matches_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_cell_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cell-area",
                transmute(notify_cell_area_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_inline_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::inline-completion",
                transmute(notify_inline_completion_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_inline_selection_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::inline-selection",
                transmute(notify_inline_selection_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_minimum_key_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::minimum-key-length",
                transmute(notify_minimum_key_length_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::model",
                transmute(notify_model_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_popup_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::popup-completion",
                transmute(notify_popup_completion_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_popup_set_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::popup-set-width",
                transmute(notify_popup_set_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_popup_single_match_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::popup-single-match",
                transmute(notify_popup_single_match_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text-column",
                transmute(notify_text_column_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn action_activated_trampoline<P>(this: *mut ffi::GtkEntryCompletion, index: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<EntryCompletion> {
    callback_guard!();
    let f: &&(Fn(&P, i32) + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked(), index)
}

unsafe extern "C" fn cursor_on_match_trampoline<P>(this: *mut ffi::GtkEntryCompletion, model: *mut ffi::GtkTreeModel, iter: *mut ffi::GtkTreeIter, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<EntryCompletion> {
    callback_guard!();
    let f: &&(Fn(&P, &TreeModel, &TreeIter) -> Inhibit + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(model), &from_glib_borrow(iter)).to_glib()
}

unsafe extern "C" fn insert_prefix_trampoline<P>(this: *mut ffi::GtkEntryCompletion, prefix: *mut libc::c_char, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<EntryCompletion> {
    callback_guard!();
    let f: &&(Fn(&P, &str) -> Inhibit + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(prefix)).to_glib()
}

unsafe extern "C" fn match_selected_trampoline<P>(this: *mut ffi::GtkEntryCompletion, model: *mut ffi::GtkTreeModel, iter: *mut ffi::GtkTreeIter, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<EntryCompletion> {
    callback_guard!();
    let f: &&(Fn(&P, &TreeModel, &TreeIter) -> Inhibit + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(model), &from_glib_borrow(iter)).to_glib()
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn no_matches_trampoline<P>(this: *mut ffi::GtkEntryCompletion, f: glib_ffi::gpointer)
where P: IsA<EntryCompletion> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_cell_area_trampoline<P>(this: *mut ffi::GtkEntryCompletion, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EntryCompletion> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_inline_completion_trampoline<P>(this: *mut ffi::GtkEntryCompletion, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EntryCompletion> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_inline_selection_trampoline<P>(this: *mut ffi::GtkEntryCompletion, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EntryCompletion> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_minimum_key_length_trampoline<P>(this: *mut ffi::GtkEntryCompletion, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EntryCompletion> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_model_trampoline<P>(this: *mut ffi::GtkEntryCompletion, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EntryCompletion> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_popup_completion_trampoline<P>(this: *mut ffi::GtkEntryCompletion, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EntryCompletion> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_popup_set_width_trampoline<P>(this: *mut ffi::GtkEntryCompletion, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EntryCompletion> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_popup_single_match_trampoline<P>(this: *mut ffi::GtkEntryCompletion, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EntryCompletion> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_column_trampoline<P>(this: *mut ffi::GtkEntryCompletion, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EntryCompletion> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked())
}
