// This file was generated by gir (4ffdbd3) from gir-files (71d73f0)
// DO NOT EDIT

use Cursor;
use Device;
use Display;
use DragProtocol;
#[cfg(feature = "v3_16")]
use Error;
#[cfg(feature = "v3_14")]
use Event;
use EventMask;
#[cfg(feature = "v3_8")]
use FrameClock;
#[cfg(feature = "v3_8")]
use FullscreenMode;
#[cfg(feature = "v3_16")]
use GLContext;
use InputSource;
use ModifierType;
use RGBA;
use Rectangle;
use Screen;
use Visual;
use WMDecoration;
use WMFunction;
use WindowEdge;
use WindowState;
use WindowType;
use WindowTypeHint;
use cairo;
use cairo_ffi;
use ffi;
use gdk_pixbuf;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Window(Object<ffi::GdkWindow>);

    match fn {
        get_type => || ffi::gdk_window_get_type(),
    }
}

impl Window {
    //pub fn add_filter<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, function: /*Unknown conversion*//*Unimplemented*/FilterFunc, data: P) {
    //    unsafe { TODO: call ffi::gdk_window_add_filter() }
    //}

    pub fn beep(&self) {
        unsafe {
            ffi::gdk_window_beep(self.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v3_22")]
    //pub fn begin_draw_frame(&self, region: /*Ignored*/&cairo::Region) -> /*Ignored*/Option<DrawingContext> {
    //    unsafe { TODO: call ffi::gdk_window_begin_draw_frame() }
    //}

    pub fn begin_move_drag(&self, button: i32, root_x: i32, root_y: i32, timestamp: u32) {
        unsafe {
            ffi::gdk_window_begin_move_drag(self.to_glib_none().0, button, root_x, root_y, timestamp);
        }
    }

    pub fn begin_move_drag_for_device<P: IsA<Device>>(&self, device: &P, button: i32, root_x: i32, root_y: i32, timestamp: u32) {
        unsafe {
            ffi::gdk_window_begin_move_drag_for_device(self.to_glib_none().0, device.to_glib_none().0, button, root_x, root_y, timestamp);
        }
    }

    pub fn begin_paint_rect(&self, rectangle: &Rectangle) {
        unsafe {
            ffi::gdk_window_begin_paint_rect(self.to_glib_none().0, rectangle.to_glib_none().0);
        }
    }

    //pub fn begin_paint_region(&self, region: /*Ignored*/&cairo::Region) {
    //    unsafe { TODO: call ffi::gdk_window_begin_paint_region() }
    //}

    pub fn begin_resize_drag(&self, edge: WindowEdge, button: i32, root_x: i32, root_y: i32, timestamp: u32) {
        unsafe {
            ffi::gdk_window_begin_resize_drag(self.to_glib_none().0, edge.to_glib(), button, root_x, root_y, timestamp);
        }
    }

    pub fn begin_resize_drag_for_device<P: IsA<Device>>(&self, edge: WindowEdge, device: &P, button: i32, root_x: i32, root_y: i32, timestamp: u32) {
        unsafe {
            ffi::gdk_window_begin_resize_drag_for_device(self.to_glib_none().0, edge.to_glib(), device.to_glib_none().0, button, root_x, root_y, timestamp);
        }
    }

    pub fn configure_finished(&self) {
        unsafe {
            ffi::gdk_window_configure_finished(self.to_glib_none().0);
        }
    }

    pub fn coords_from_parent(&self, parent_x: f64, parent_y: f64) -> (f64, f64) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            ffi::gdk_window_coords_from_parent(self.to_glib_none().0, parent_x, parent_y, &mut x, &mut y);
            (x, y)
        }
    }

    pub fn coords_to_parent(&self, x: f64, y: f64) -> (f64, f64) {
        unsafe {
            let mut parent_x = mem::uninitialized();
            let mut parent_y = mem::uninitialized();
            ffi::gdk_window_coords_to_parent(self.to_glib_none().0, x, y, &mut parent_x, &mut parent_y);
            (parent_x, parent_y)
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn create_gl_context(&self) -> Result<GLContext, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_window_create_gl_context(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn create_similar_image_surface(&self, format: i32, width: i32, height: i32, scale: i32) -> Option<cairo::Surface> {
        unsafe {
            from_glib_full(ffi::gdk_window_create_similar_image_surface(self.to_glib_none().0, format, width, height, scale))
        }
    }

    //pub fn create_similar_surface(&self, content: /*Ignored*/cairo::Content, width: i32, height: i32) -> Option<cairo::Surface> {
    //    unsafe { TODO: call ffi::gdk_window_create_similar_surface() }
    //}

    pub fn deiconify(&self) {
        unsafe {
            ffi::gdk_window_deiconify(self.to_glib_none().0);
        }
    }

    pub fn destroy(&self) {
        unsafe {
            ffi::gdk_window_destroy(self.to_glib_none().0);
        }
    }

    pub fn destroy_notify(&self) {
        unsafe {
            ffi::gdk_window_destroy_notify(self.to_glib_none().0);
        }
    }

    pub fn enable_synchronized_configure(&self) {
        unsafe {
            ffi::gdk_window_enable_synchronized_configure(self.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v3_22")]
    //pub fn end_draw_frame(&self, context: /*Ignored*/&DrawingContext) {
    //    unsafe { TODO: call ffi::gdk_window_end_draw_frame() }
    //}

    pub fn end_paint(&self) {
        unsafe {
            ffi::gdk_window_end_paint(self.to_glib_none().0);
        }
    }

    pub fn ensure_native(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_ensure_native(self.to_glib_none().0))
        }
    }

    pub fn flush(&self) {
        unsafe {
            ffi::gdk_window_flush(self.to_glib_none().0);
        }
    }

    pub fn focus(&self, timestamp: u32) {
        unsafe {
            ffi::gdk_window_focus(self.to_glib_none().0, timestamp);
        }
    }

    pub fn freeze_toplevel_updates_libgtk_only(&self) {
        unsafe {
            ffi::gdk_window_freeze_toplevel_updates_libgtk_only(self.to_glib_none().0);
        }
    }

    pub fn freeze_updates(&self) {
        unsafe {
            ffi::gdk_window_freeze_updates(self.to_glib_none().0);
        }
    }

    pub fn fullscreen(&self) {
        unsafe {
            ffi::gdk_window_fullscreen(self.to_glib_none().0);
        }
    }

    pub fn fullscreen_on_monitor(&self, monitor: i32) {
        unsafe {
            ffi::gdk_window_fullscreen_on_monitor(self.to_glib_none().0, monitor);
        }
    }

    pub fn geometry_changed(&self) {
        unsafe {
            ffi::gdk_window_geometry_changed(self.to_glib_none().0);
        }
    }

    pub fn get_accept_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_get_accept_focus(self.to_glib_none().0))
        }
    }

    //pub fn get_background_pattern(&self) -> /*Ignored*/Option<cairo::Pattern> {
    //    unsafe { TODO: call ffi::gdk_window_get_background_pattern() }
    //}

    pub fn get_children(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_window_get_children(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v3_10")]
    //pub fn get_children_with_user_data<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, user_data: P) -> Vec<Window> {
    //    unsafe { TODO: call ffi::gdk_window_get_children_with_user_data() }
    //}

    //pub fn get_clip_region(&self) -> /*Ignored*/Option<cairo::Region> {
    //    unsafe { TODO: call ffi::gdk_window_get_clip_region() }
    //}

    pub fn get_composited(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_get_composited(self.to_glib_none().0))
        }
    }

    pub fn get_cursor(&self) -> Option<Cursor> {
        unsafe {
            from_glib_none(ffi::gdk_window_get_cursor(self.to_glib_none().0))
        }
    }

    pub fn get_decorations(&self) -> Option<WMDecoration> {
        unsafe {
            let mut decorations = mem::uninitialized();
            let ret = from_glib(ffi::gdk_window_get_decorations(self.to_glib_none().0, &mut decorations));
            if ret { Some(from_glib(decorations)) } else { None }
        }
    }

    pub fn get_device_cursor<P: IsA<Device>>(&self, device: &P) -> Option<Cursor> {
        unsafe {
            from_glib_none(ffi::gdk_window_get_device_cursor(self.to_glib_none().0, device.to_glib_none().0))
        }
    }

    pub fn get_device_events<P: IsA<Device>>(&self, device: &P) -> EventMask {
        unsafe {
            from_glib(ffi::gdk_window_get_device_events(self.to_glib_none().0, device.to_glib_none().0))
        }
    }

    pub fn get_device_position<P: IsA<Device>>(&self, device: &P) -> (Option<Window>, i32, i32, ModifierType) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let mut mask = mem::uninitialized();
            let ret = from_glib_none(ffi::gdk_window_get_device_position(self.to_glib_none().0, device.to_glib_none().0, &mut x, &mut y, &mut mask));
            (ret, x, y, from_glib(mask))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_device_position_double<P: IsA<Device>>(&self, device: &P) -> (Option<Window>, f64, f64, ModifierType) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let mut mask = mem::uninitialized();
            let ret = from_glib_none(ffi::gdk_window_get_device_position_double(self.to_glib_none().0, device.to_glib_none().0, &mut x, &mut y, &mut mask));
            (ret, x, y, from_glib(mask))
        }
    }

    pub fn get_display(&self) -> Display {
        unsafe {
            from_glib_none(ffi::gdk_window_get_display(self.to_glib_none().0))
        }
    }

    pub fn get_drag_protocol(&self) -> (DragProtocol, Window) {
        unsafe {
            let mut target = ptr::null_mut();
            let ret = from_glib(ffi::gdk_window_get_drag_protocol(self.to_glib_none().0, &mut target));
            (ret, from_glib_full(target))
        }
    }

    pub fn get_effective_parent(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gdk_window_get_effective_parent(self.to_glib_none().0))
        }
    }

    pub fn get_effective_toplevel(&self) -> Window {
        unsafe {
            from_glib_none(ffi::gdk_window_get_effective_toplevel(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn get_event_compression(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_get_event_compression(self.to_glib_none().0))
        }
    }

    pub fn get_events(&self) -> EventMask {
        unsafe {
            from_glib(ffi::gdk_window_get_events(self.to_glib_none().0))
        }
    }

    pub fn get_focus_on_map(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_get_focus_on_map(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_frame_clock(&self) -> Option<FrameClock> {
        unsafe {
            from_glib_none(ffi::gdk_window_get_frame_clock(self.to_glib_none().0))
        }
    }

    pub fn get_frame_extents(&self) -> Rectangle {
        unsafe {
            let mut rect = Rectangle::uninitialized();
            ffi::gdk_window_get_frame_extents(self.to_glib_none().0, rect.to_glib_none_mut().0);
            rect
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_fullscreen_mode(&self) -> FullscreenMode {
        unsafe {
            from_glib(ffi::gdk_window_get_fullscreen_mode(self.to_glib_none().0))
        }
    }

    pub fn get_geometry(&self) -> (i32, i32, i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gdk_window_get_geometry(self.to_glib_none().0, &mut x, &mut y, &mut width, &mut height);
            (x, y, width, height)
        }
    }

    pub fn get_group(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gdk_window_get_group(self.to_glib_none().0))
        }
    }

    pub fn get_height(&self) -> i32 {
        unsafe {
            ffi::gdk_window_get_height(self.to_glib_none().0)
        }
    }

    pub fn get_modal_hint(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_get_modal_hint(self.to_glib_none().0))
        }
    }

    pub fn get_origin(&self) -> (i32, i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let ret = ffi::gdk_window_get_origin(self.to_glib_none().0, &mut x, &mut y);
            (ret, x, y)
        }
    }

    pub fn get_parent(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gdk_window_get_parent(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_18")]
    pub fn get_pass_through(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_get_pass_through(self.to_glib_none().0))
        }
    }

    pub fn get_pointer(&self) -> (Option<Window>, i32, i32, ModifierType) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let mut mask = mem::uninitialized();
            let ret = from_glib_none(ffi::gdk_window_get_pointer(self.to_glib_none().0, &mut x, &mut y, &mut mask));
            (ret, x, y, from_glib(mask))
        }
    }

    pub fn get_position(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            ffi::gdk_window_get_position(self.to_glib_none().0, &mut x, &mut y);
            (x, y)
        }
    }

    pub fn get_root_coords(&self, x: i32, y: i32) -> (i32, i32) {
        unsafe {
            let mut root_x = mem::uninitialized();
            let mut root_y = mem::uninitialized();
            ffi::gdk_window_get_root_coords(self.to_glib_none().0, x, y, &mut root_x, &mut root_y);
            (root_x, root_y)
        }
    }

    pub fn get_root_origin(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            ffi::gdk_window_get_root_origin(self.to_glib_none().0, &mut x, &mut y);
            (x, y)
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_scale_factor(&self) -> i32 {
        unsafe {
            ffi::gdk_window_get_scale_factor(self.to_glib_none().0)
        }
    }

    pub fn get_screen(&self) -> Screen {
        unsafe {
            from_glib_none(ffi::gdk_window_get_screen(self.to_glib_none().0))
        }
    }

    pub fn get_source_events(&self, source: InputSource) -> EventMask {
        unsafe {
            from_glib(ffi::gdk_window_get_source_events(self.to_glib_none().0, source.to_glib()))
        }
    }

    pub fn get_state(&self) -> WindowState {
        unsafe {
            from_glib(ffi::gdk_window_get_state(self.to_glib_none().0))
        }
    }

    pub fn get_support_multidevice(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_get_support_multidevice(self.to_glib_none().0))
        }
    }

    pub fn get_toplevel(&self) -> Window {
        unsafe {
            from_glib_none(ffi::gdk_window_get_toplevel(self.to_glib_none().0))
        }
    }

    pub fn get_type_hint(&self) -> WindowTypeHint {
        unsafe {
            from_glib(ffi::gdk_window_get_type_hint(self.to_glib_none().0))
        }
    }

    //pub fn get_update_area(&self) -> /*Ignored*/Option<cairo::Region> {
    //    unsafe { TODO: call ffi::gdk_window_get_update_area() }
    //}

    //pub fn get_user_data(&self, data: /*Unimplemented*/&mut Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gdk_window_get_user_data() }
    //}

    //pub fn get_visible_region(&self) -> /*Ignored*/Option<cairo::Region> {
    //    unsafe { TODO: call ffi::gdk_window_get_visible_region() }
    //}

    pub fn get_visual(&self) -> Visual {
        unsafe {
            from_glib_none(ffi::gdk_window_get_visual(self.to_glib_none().0))
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe {
            ffi::gdk_window_get_width(self.to_glib_none().0)
        }
    }

    pub fn get_window_type(&self) -> WindowType {
        unsafe {
            from_glib(ffi::gdk_window_get_window_type(self.to_glib_none().0))
        }
    }

    pub fn has_native(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_has_native(self.to_glib_none().0))
        }
    }

    pub fn hide(&self) {
        unsafe {
            ffi::gdk_window_hide(self.to_glib_none().0);
        }
    }

    pub fn iconify(&self) {
        unsafe {
            ffi::gdk_window_iconify(self.to_glib_none().0);
        }
    }

    //pub fn input_shape_combine_region(&self, shape_region: /*Ignored*/&cairo::Region, offset_x: i32, offset_y: i32) {
    //    unsafe { TODO: call ffi::gdk_window_input_shape_combine_region() }
    //}

    //pub fn invalidate_maybe_recurse<'a, P: Into<Option<&'a /*Unimplemented*/WindowChildFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, region: /*Ignored*/&cairo::Region, child_func: P, user_data: Q) {
    //    unsafe { TODO: call ffi::gdk_window_invalidate_maybe_recurse() }
    //}

    pub fn invalidate_rect<'a, P: Into<Option<&'a Rectangle>>>(&self, rect: P, invalidate_children: bool) {
        let rect = rect.into();
        let rect = rect.to_glib_none().0;
        unsafe {
            ffi::gdk_window_invalidate_rect(self.to_glib_none().0, rect, invalidate_children.to_glib());
        }
    }

    //pub fn invalidate_region(&self, region: /*Ignored*/&cairo::Region, invalidate_children: bool) {
    //    unsafe { TODO: call ffi::gdk_window_invalidate_region() }
    //}

    pub fn is_destroyed(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_is_destroyed(self.to_glib_none().0))
        }
    }

    pub fn is_input_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_is_input_only(self.to_glib_none().0))
        }
    }

    pub fn is_shaped(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_is_shaped(self.to_glib_none().0))
        }
    }

    pub fn is_viewable(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_is_viewable(self.to_glib_none().0))
        }
    }

    pub fn is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_is_visible(self.to_glib_none().0))
        }
    }

    pub fn lower(&self) {
        unsafe {
            ffi::gdk_window_lower(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn mark_paint_from_clip(&self, cr: &cairo::Context) {
        unsafe {
            ffi::gdk_window_mark_paint_from_clip(self.to_glib_none().0, mut_override(cr.to_glib_none().0));
        }
    }

    pub fn maximize(&self) {
        unsafe {
            ffi::gdk_window_maximize(self.to_glib_none().0);
        }
    }

    pub fn merge_child_input_shapes(&self) {
        unsafe {
            ffi::gdk_window_merge_child_input_shapes(self.to_glib_none().0);
        }
    }

    pub fn merge_child_shapes(&self) {
        unsafe {
            ffi::gdk_window_merge_child_shapes(self.to_glib_none().0);
        }
    }

    pub fn move_(&self, x: i32, y: i32) {
        unsafe {
            ffi::gdk_window_move(self.to_glib_none().0, x, y);
        }
    }

    //pub fn move_region(&self, region: /*Ignored*/&cairo::Region, dx: i32, dy: i32) {
    //    unsafe { TODO: call ffi::gdk_window_move_region() }
    //}

    pub fn move_resize(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ffi::gdk_window_move_resize(self.to_glib_none().0, x, y, width, height);
        }
    }

    pub fn peek_children(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gdk_window_peek_children(self.to_glib_none().0))
        }
    }

    pub fn process_updates(&self, update_children: bool) {
        unsafe {
            ffi::gdk_window_process_updates(self.to_glib_none().0, update_children.to_glib());
        }
    }

    pub fn raise(&self) {
        unsafe {
            ffi::gdk_window_raise(self.to_glib_none().0);
        }
    }

    pub fn register_dnd(&self) {
        unsafe {
            ffi::gdk_window_register_dnd(self.to_glib_none().0);
        }
    }

    //pub fn remove_filter<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, function: /*Unknown conversion*//*Unimplemented*/FilterFunc, data: P) {
    //    unsafe { TODO: call ffi::gdk_window_remove_filter() }
    //}

    pub fn reparent(&self, new_parent: &Window, x: i32, y: i32) {
        unsafe {
            ffi::gdk_window_reparent(self.to_glib_none().0, new_parent.to_glib_none().0, x, y);
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            ffi::gdk_window_resize(self.to_glib_none().0, width, height);
        }
    }

    pub fn restack<'a, P: Into<Option<&'a Window>>>(&self, sibling: P, above: bool) {
        let sibling = sibling.into();
        let sibling = sibling.to_glib_none().0;
        unsafe {
            ffi::gdk_window_restack(self.to_glib_none().0, sibling, above.to_glib());
        }
    }

    pub fn scroll(&self, dx: i32, dy: i32) {
        unsafe {
            ffi::gdk_window_scroll(self.to_glib_none().0, dx, dy);
        }
    }

    pub fn set_accept_focus(&self, accept_focus: bool) {
        unsafe {
            ffi::gdk_window_set_accept_focus(self.to_glib_none().0, accept_focus.to_glib());
        }
    }

    //pub fn set_background(&self, color: /*Ignored*/&Color) {
    //    unsafe { TODO: call ffi::gdk_window_set_background() }
    //}

    //pub fn set_background_pattern<'a, P: Into<Option<&'a /*Ignored*/cairo::Pattern>>>(&self, pattern: P) {
    //    unsafe { TODO: call ffi::gdk_window_set_background_pattern() }
    //}

    pub fn set_background_rgba(&self, rgba: &RGBA) {
        unsafe {
            ffi::gdk_window_set_background_rgba(self.to_glib_none().0, rgba.to_glib_none().0);
        }
    }

    pub fn set_child_input_shapes(&self) {
        unsafe {
            ffi::gdk_window_set_child_input_shapes(self.to_glib_none().0);
        }
    }

    pub fn set_child_shapes(&self) {
        unsafe {
            ffi::gdk_window_set_child_shapes(self.to_glib_none().0);
        }
    }

    pub fn set_composited(&self, composited: bool) {
        unsafe {
            ffi::gdk_window_set_composited(self.to_glib_none().0, composited.to_glib());
        }
    }

    pub fn set_cursor<'a, P: Into<Option<&'a Cursor>>>(&self, cursor: P) {
        let cursor = cursor.into();
        let cursor = cursor.to_glib_none().0;
        unsafe {
            ffi::gdk_window_set_cursor(self.to_glib_none().0, cursor);
        }
    }

    pub fn set_decorations(&self, decorations: WMDecoration) {
        unsafe {
            ffi::gdk_window_set_decorations(self.to_glib_none().0, decorations.to_glib());
        }
    }

    pub fn set_device_cursor<P: IsA<Device>>(&self, device: &P, cursor: &Cursor) {
        unsafe {
            ffi::gdk_window_set_device_cursor(self.to_glib_none().0, device.to_glib_none().0, cursor.to_glib_none().0);
        }
    }

    pub fn set_device_events<P: IsA<Device>>(&self, device: &P, event_mask: EventMask) {
        unsafe {
            ffi::gdk_window_set_device_events(self.to_glib_none().0, device.to_glib_none().0, event_mask.to_glib());
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn set_event_compression(&self, event_compression: bool) {
        unsafe {
            ffi::gdk_window_set_event_compression(self.to_glib_none().0, event_compression.to_glib());
        }
    }

    pub fn set_events(&self, event_mask: EventMask) {
        unsafe {
            ffi::gdk_window_set_events(self.to_glib_none().0, event_mask.to_glib());
        }
    }

    pub fn set_focus_on_map(&self, focus_on_map: bool) {
        unsafe {
            ffi::gdk_window_set_focus_on_map(self.to_glib_none().0, focus_on_map.to_glib());
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn set_fullscreen_mode(&self, mode: FullscreenMode) {
        unsafe {
            ffi::gdk_window_set_fullscreen_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    pub fn set_functions(&self, functions: WMFunction) {
        unsafe {
            ffi::gdk_window_set_functions(self.to_glib_none().0, functions.to_glib());
        }
    }

    //pub fn set_geometry_hints(&self, geometry: /*Ignored*/&Geometry, geom_mask: WindowHints) {
    //    unsafe { TODO: call ffi::gdk_window_set_geometry_hints() }
    //}

    pub fn set_group<'a, P: Into<Option<&'a Window>>>(&self, leader: P) {
        let leader = leader.into();
        let leader = leader.to_glib_none().0;
        unsafe {
            ffi::gdk_window_set_group(self.to_glib_none().0, leader);
        }
    }

    pub fn set_icon_list(&self, pixbufs: &[gdk_pixbuf::Pixbuf]) {
        unsafe {
            ffi::gdk_window_set_icon_list(self.to_glib_none().0, pixbufs.to_glib_none().0);
        }
    }

    pub fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, name: P) {
        let name = name.into();
        let name = name.to_glib_none().0;
        unsafe {
            ffi::gdk_window_set_icon_name(self.to_glib_none().0, name);
        }
    }

    //#[cfg(feature = "v3_10")]
    //pub fn set_invalidate_handler(&self, handler: /*Unknown conversion*//*Unimplemented*/WindowInvalidateHandlerFunc) {
    //    unsafe { TODO: call ffi::gdk_window_set_invalidate_handler() }
    //}

    pub fn set_keep_above(&self, setting: bool) {
        unsafe {
            ffi::gdk_window_set_keep_above(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_keep_below(&self, setting: bool) {
        unsafe {
            ffi::gdk_window_set_keep_below(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_modal_hint(&self, modal: bool) {
        unsafe {
            ffi::gdk_window_set_modal_hint(self.to_glib_none().0, modal.to_glib());
        }
    }

    pub fn set_opacity(&self, opacity: f64) {
        unsafe {
            ffi::gdk_window_set_opacity(self.to_glib_none().0, opacity);
        }
    }

    //#[cfg(feature = "v3_10")]
    //pub fn set_opaque_region<'a, P: Into<Option<&'a /*Ignored*/cairo::Region>>>(&self, region: P) {
    //    unsafe { TODO: call ffi::gdk_window_set_opaque_region() }
    //}

    pub fn set_override_redirect(&self, override_redirect: bool) {
        unsafe {
            ffi::gdk_window_set_override_redirect(self.to_glib_none().0, override_redirect.to_glib());
        }
    }

    #[cfg(feature = "v3_18")]
    pub fn set_pass_through(&self, pass_through: bool) {
        unsafe {
            ffi::gdk_window_set_pass_through(self.to_glib_none().0, pass_through.to_glib());
        }
    }

    pub fn set_role(&self, role: &str) {
        unsafe {
            ffi::gdk_window_set_role(self.to_glib_none().0, role.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn set_shadow_width(&self, left: i32, right: i32, top: i32, bottom: i32) {
        unsafe {
            ffi::gdk_window_set_shadow_width(self.to_glib_none().0, left, right, top, bottom);
        }
    }

    pub fn set_skip_pager_hint(&self, skips_pager: bool) {
        unsafe {
            ffi::gdk_window_set_skip_pager_hint(self.to_glib_none().0, skips_pager.to_glib());
        }
    }

    pub fn set_skip_taskbar_hint(&self, skips_taskbar: bool) {
        unsafe {
            ffi::gdk_window_set_skip_taskbar_hint(self.to_glib_none().0, skips_taskbar.to_glib());
        }
    }

    pub fn set_source_events(&self, source: InputSource, event_mask: EventMask) {
        unsafe {
            ffi::gdk_window_set_source_events(self.to_glib_none().0, source.to_glib(), event_mask.to_glib());
        }
    }

    pub fn set_startup_id(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_window_set_startup_id(self.to_glib_none().0, startup_id.to_glib_none().0);
        }
    }

    pub fn set_static_gravities(&self, use_static: bool) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_set_static_gravities(self.to_glib_none().0, use_static.to_glib()))
        }
    }

    pub fn set_support_multidevice(&self, support_multidevice: bool) {
        unsafe {
            ffi::gdk_window_set_support_multidevice(self.to_glib_none().0, support_multidevice.to_glib());
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gdk_window_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn set_transient_for(&self, parent: &Window) {
        unsafe {
            ffi::gdk_window_set_transient_for(self.to_glib_none().0, parent.to_glib_none().0);
        }
    }

    pub fn set_type_hint(&self, hint: WindowTypeHint) {
        unsafe {
            ffi::gdk_window_set_type_hint(self.to_glib_none().0, hint.to_glib());
        }
    }

    pub fn set_urgency_hint(&self, urgent: bool) {
        unsafe {
            ffi::gdk_window_set_urgency_hint(self.to_glib_none().0, urgent.to_glib());
        }
    }

    //pub fn set_user_data<'a, P: IsA</*Ignored*/glib::Object> + 'a, Q: Into<Option<&'a P>>>(&self, user_data: Q) {
    //    unsafe { TODO: call ffi::gdk_window_set_user_data() }
    //}

    //pub fn shape_combine_region<'a, P: Into<Option<&'a /*Ignored*/cairo::Region>>>(&self, shape_region: P, offset_x: i32, offset_y: i32) {
    //    unsafe { TODO: call ffi::gdk_window_shape_combine_region() }
    //}

    pub fn show(&self) {
        unsafe {
            ffi::gdk_window_show(self.to_glib_none().0);
        }
    }

    pub fn show_unraised(&self) {
        unsafe {
            ffi::gdk_window_show_unraised(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn show_window_menu(&self, event: &mut Event) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_show_window_menu(self.to_glib_none().0, event.to_glib_none_mut().0))
        }
    }

    pub fn stick(&self) {
        unsafe {
            ffi::gdk_window_stick(self.to_glib_none().0);
        }
    }

    pub fn thaw_toplevel_updates_libgtk_only(&self) {
        unsafe {
            ffi::gdk_window_thaw_toplevel_updates_libgtk_only(self.to_glib_none().0);
        }
    }

    pub fn thaw_updates(&self) {
        unsafe {
            ffi::gdk_window_thaw_updates(self.to_glib_none().0);
        }
    }

    pub fn unfullscreen(&self) {
        unsafe {
            ffi::gdk_window_unfullscreen(self.to_glib_none().0);
        }
    }

    pub fn unmaximize(&self) {
        unsafe {
            ffi::gdk_window_unmaximize(self.to_glib_none().0);
        }
    }

    pub fn unstick(&self) {
        unsafe {
            ffi::gdk_window_unstick(self.to_glib_none().0);
        }
    }

    pub fn withdraw(&self) {
        unsafe {
            ffi::gdk_window_withdraw(self.to_glib_none().0);
        }
    }

    pub fn at_pointer() -> (Window, i32, i32) {
        assert_initialized_main_thread!();
        unsafe {
            let mut win_x = mem::uninitialized();
            let mut win_y = mem::uninitialized();
            let ret = from_glib_none(ffi::gdk_window_at_pointer(&mut win_x, &mut win_y));
            (ret, win_x, win_y)
        }
    }

    //pub fn constrain_size(geometry: /*Ignored*/&mut Geometry, flags: WindowHints, width: i32, height: i32) -> (i32, i32) {
    //    unsafe { TODO: call ffi::gdk_window_constrain_size() }
    //}

    pub fn process_all_updates() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_window_process_all_updates();
        }
    }

    pub fn set_debug_updates(setting: bool) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_window_set_debug_updates(setting.to_glib());
        }
    }

    pub fn connect_create_surface<F: Fn(&Window, i32, i32) -> cairo::Surface + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Window, i32, i32) -> cairo::Surface + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "create-surface",
                transmute(create_surface_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //pub fn connect_from_embedder<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Out offscreen_x: *.Double
    //    Out offscreen_y: *.Double
    //}

    //#[cfg(feature = "v3_22")]
    //pub fn connect_moved_to_rect<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Unimplemented flipped_rect: *.Pointer
    //    Unimplemented final_rect: *.Pointer
    //}

    pub fn connect_pick_embedded_child<F: Fn(&Window, f64, f64) -> Option<Window> + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Window, f64, f64) -> Option<Window> + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "pick-embedded-child",
                transmute(pick_embedded_child_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //pub fn connect_to_embedder<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Out embedder_x: *.Double
    //    Out embedder_y: *.Double
    //}
}

unsafe extern "C" fn create_surface_trampoline(this: *mut ffi::GdkWindow, width: libc::c_int, height: libc::c_int, f: glib_ffi::gpointer) -> *mut cairo_ffi::cairo_surface_t {
    callback_guard!();
    let f: &Box_<Fn(&Window, i32, i32) -> cairo::Surface + 'static> = transmute(f);
    f(&from_glib_none(this), width, height).to_glib_full()
}

unsafe extern "C" fn pick_embedded_child_trampoline(this: *mut ffi::GdkWindow, x: libc::c_double, y: libc::c_double, f: glib_ffi::gpointer) -> *mut ffi::GdkWindow {
    callback_guard!();
    let f: &Box_<Fn(&Window, f64, f64) -> Option<Window> + 'static> = transmute(f);
    f(&from_glib_none(this), x, y)/*Not checked*/.to_glib_none().0
}
