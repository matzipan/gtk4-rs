// Take a look at the license at the top of the repository in the LICENSE file.

use libc::{c_char, c_int};
use std::mem;

use crate::subclass::prelude::*;
use crate::{CellEditable, CellRenderer, CellRendererState, SizeRequestMode, Snapshot, Widget};
use glib::object::IsA;
use glib::translate::*;
use glib::{Cast, GString, Object};

pub trait CellRendererImpl: CellRendererImplExt + ObjectImpl {
    fn activate<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        event: Option<&gdk::Event>,
        widget: &P,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool {
        self.parent_activate(
            renderer,
            event,
            widget,
            path,
            background_area,
            cell_area,
            flags,
        )
    }

    fn editing_canceled(&self, renderer: &Self::Type) {
        self.parent_editing_canceled(renderer)
    }

    fn editing_started(&self, renderer: &Self::Type, editable: &CellEditable, path: &str) {
        self.parent_editing_started(renderer, editable, path)
    }

    #[doc(alias = "get_aligned_area")]
    fn aligned_area<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        widget: &P,
        flags: CellRendererState,
        cell_area: &gdk::Rectangle,
    ) -> gdk::Rectangle {
        self.parent_aligned_area(renderer, widget, flags, cell_area)
    }

    #[doc(alias = "get_preferred_height_for_width")]
    fn preferred_height_for_width<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        widget: &P,
        width: i32,
    ) -> (i32, i32) {
        self.parent_preferred_height_for_width(renderer, widget, width)
    }

    #[doc(alias = "get_preferred_height")]
    fn preferred_height<P: IsA<Widget>>(&self, renderer: &Self::Type, widget: &P) -> (i32, i32) {
        self.parent_preferred_height(renderer, widget)
    }

    #[doc(alias = "get_preferred_width_for_height")]
    fn preferred_width_for_height<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        widget: &P,
        height: i32,
    ) -> (i32, i32) {
        self.parent_preferred_width_for_height(renderer, widget, height)
    }

    #[doc(alias = "get_preferred_width")]
    fn preferred_width<P: IsA<Widget>>(&self, renderer: &Self::Type, widget: &P) -> (i32, i32) {
        self.parent_preferred_width(renderer, widget)
    }

    #[doc(alias = "get_request_mode")]
    fn request_mode(&self, renderer: &Self::Type) -> SizeRequestMode {
        self.parent_request_mode(renderer)
    }

    fn snapshot<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        snapshot: &Snapshot,
        widget: &P,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) {
        self.parent_snapshot(
            renderer,
            snapshot,
            widget,
            background_area,
            cell_area,
            flags,
        );
    }

    fn start_editing<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        event: Option<&gdk::Event>,
        widget: &P,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> Option<CellEditable> {
        self.parent_start_editing(
            renderer,
            event,
            widget,
            path,
            background_area,
            cell_area,
            flags,
        )
    }
}

pub trait CellRendererImplExt: ObjectSubclass {
    fn parent_activate<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        event: Option<&gdk::Event>,
        widget: &P,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool;
    fn parent_editing_canceled(&self, renderer: &Self::Type);
    fn parent_editing_started(&self, renderer: &Self::Type, editable: &CellEditable, path: &str);
    fn parent_aligned_area<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        widget: &P,
        flags: CellRendererState,
        cell_area: &gdk::Rectangle,
    ) -> gdk::Rectangle;
    fn parent_preferred_height_for_width<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        widget: &P,
        width: i32,
    ) -> (i32, i32);
    fn parent_preferred_height<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        widget: &P,
    ) -> (i32, i32);
    fn parent_preferred_width_for_height<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        widget: &P,
        height: i32,
    ) -> (i32, i32);
    fn parent_preferred_width<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        widget: &P,
    ) -> (i32, i32);
    fn parent_request_mode(&self, renderer: &Self::Type) -> SizeRequestMode;
    fn parent_snapshot<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        snapshot: &Snapshot,
        widget: &P,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    );
    fn parent_start_editing<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        event: Option<&gdk::Event>,
        widget: &P,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> Option<CellEditable>;
}

impl<T: CellRendererImpl> CellRendererImplExt for T {
    fn parent_request_mode(&self, renderer: &Self::Type) -> SizeRequestMode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellRendererClass;
            let f = (*parent_class).get_request_mode.unwrap();
            from_glib(f(renderer
                .unsafe_cast_ref::<CellRenderer>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_preferred_width<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        widget: &P,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellRendererClass;
            let f = (*parent_class).get_preferred_width.unwrap();

            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                renderer.unsafe_cast_ref::<CellRenderer>().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    fn parent_preferred_width_for_height<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        widget: &P,
        height: i32,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellRendererClass;
            let f = (*parent_class).get_preferred_width_for_height.unwrap();

            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                renderer.unsafe_cast_ref::<CellRenderer>().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                height,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }
    fn parent_preferred_height<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        widget: &P,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellRendererClass;
            let f = (*parent_class).get_preferred_height.unwrap();
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                renderer.unsafe_cast_ref::<CellRenderer>().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }
    fn parent_preferred_height_for_width<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        widget: &P,
        width: i32,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellRendererClass;
            let f = (*parent_class).get_preferred_height_for_width.unwrap();
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                renderer.unsafe_cast_ref::<CellRenderer>().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                width,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    fn parent_aligned_area<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        widget: &P,
        flags: CellRendererState,
        cell_area: &gdk::Rectangle,
    ) -> gdk::Rectangle {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellRendererClass;
            let mut aligned_area = gdk::Rectangle::uninitialized();
            let f = (*parent_class).get_aligned_area.unwrap();
            f(
                renderer.unsafe_cast_ref::<CellRenderer>().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                flags.into_glib(),
                cell_area.to_glib_none().0,
                aligned_area.to_glib_none_mut().0,
            );
            aligned_area
        }
    }

    fn parent_snapshot<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        snapshot: &Snapshot,
        widget: &P,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellRendererClass;
            if let Some(f) = (*parent_class).snapshot {
                f(
                    renderer.unsafe_cast_ref::<CellRenderer>().to_glib_none().0,
                    snapshot.to_glib_none().0,
                    widget.as_ref().to_glib_none().0,
                    background_area.to_glib_none().0,
                    cell_area.to_glib_none().0,
                    flags.into_glib(),
                )
            }
        }
    }

    fn parent_activate<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        event: Option<&gdk::Event>,
        widget: &P,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellRendererClass;
            if let Some(f) = (*parent_class).activate {
                from_glib(f(
                    renderer.unsafe_cast_ref::<CellRenderer>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                    widget.as_ref().to_glib_none().0,
                    path.to_glib_none().0,
                    background_area.to_glib_none().0,
                    cell_area.to_glib_none().0,
                    flags.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_start_editing<P: IsA<Widget>>(
        &self,
        renderer: &Self::Type,
        event: Option<&gdk::Event>,
        widget: &P,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> Option<CellEditable> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellRendererClass;
            if let Some(f) = (*parent_class).start_editing {
                from_glib_none(f(
                    renderer.unsafe_cast_ref::<CellRenderer>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                    widget.as_ref().to_glib_none().0,
                    path.to_glib_none().0,
                    background_area.to_glib_none().0,
                    cell_area.to_glib_none().0,
                    flags.into_glib(),
                ))
            } else {
                None
            }
        }
    }

    fn parent_editing_canceled(&self, renderer: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellRendererClass;
            if let Some(f) = (*parent_class).editing_canceled {
                f(renderer.unsafe_cast_ref::<CellRenderer>().to_glib_none().0)
            }
        }
    }

    fn parent_editing_started(&self, renderer: &Self::Type, editable: &CellEditable, path: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellRendererClass;
            if let Some(f) = (*parent_class).editing_started {
                f(
                    renderer.unsafe_cast_ref::<CellRenderer>().to_glib_none().0,
                    editable.to_glib_none().0,
                    path.to_glib_none().0,
                )
            }
        }
    }
}

unsafe impl<T: CellRendererImpl> IsSubclassable<T> for CellRenderer {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <Object as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();

        klass.activate = Some(cell_renderer_activate::<T>);
        klass.editing_canceled = Some(cell_renderer_editing_canceled::<T>);
        klass.editing_started = Some(cell_renderer_editing_started::<T>);
        klass.get_aligned_area = Some(cell_renderer_get_aligned_area::<T>);
        klass.get_preferred_height_for_width =
            Some(cell_renderer_get_preferred_height_for_width::<T>);
        klass.get_preferred_height = Some(cell_renderer_get_preferred_height::<T>);
        klass.get_preferred_width_for_height =
            Some(cell_renderer_get_preferred_width_for_height::<T>);
        klass.get_preferred_width = Some(cell_renderer_get_preferred_width::<T>);
        klass.get_request_mode = Some(cell_renderer_get_request_mode::<T>);
        klass.snapshot = Some(cell_renderer_snapshot::<T>);
        klass.start_editing = Some(cell_renderer_start_editing::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn cell_renderer_activate<T: CellRendererImpl>(
    ptr: *mut ffi::GtkCellRenderer,
    evtptr: *mut gdk::ffi::GdkEvent,
    wdgtptr: *mut ffi::GtkWidget,
    pathptr: *const c_char,
    bgptr: *const gdk::ffi::GdkRectangle,
    cellptr: *const gdk::ffi::GdkRectangle,
    flags: ffi::GtkCellRendererState,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);
    let evt: Borrowed<Option<gdk::Event>> = from_glib_borrow(evtptr);

    imp.activate(
        wrap.unsafe_cast_ref(),
        evt.as_ref().as_ref(),
        &*widget,
        &GString::from_glib_borrow(pathptr),
        &from_glib_borrow(bgptr),
        &from_glib_borrow(cellptr),
        from_glib(flags),
    )
    .into_glib()
}

unsafe extern "C" fn cell_renderer_editing_canceled<T: CellRendererImpl>(
    ptr: *mut ffi::GtkCellRenderer,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);

    imp.editing_canceled(wrap.unsafe_cast_ref());
}

unsafe extern "C" fn cell_renderer_editing_started<T: CellRendererImpl>(
    ptr: *mut ffi::GtkCellRenderer,
    editableptr: *mut ffi::GtkCellEditable,
    pathptr: *const c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let editable = from_glib_borrow(editableptr);

    imp.editing_started(
        wrap.unsafe_cast_ref(),
        &editable,
        &GString::from_glib_borrow(pathptr),
    );
}

unsafe extern "C" fn cell_renderer_get_aligned_area<T: CellRendererImpl>(
    ptr: *mut ffi::GtkCellRenderer,
    wdgtptr: *mut ffi::GtkWidget,
    flags: ffi::GtkCellRendererState,
    cellarea: *const gdk::ffi::GdkRectangle,
    alignedptr: *mut gdk::ffi::GdkRectangle,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    let rectangle = imp.aligned_area(
        wrap.unsafe_cast_ref(),
        &*widget,
        from_glib(flags),
        &from_glib_borrow(cellarea),
    );
    *alignedptr = *rectangle.to_glib_none().0;
}

unsafe extern "C" fn cell_renderer_get_preferred_height_for_width<T: CellRendererImpl>(
    ptr: *mut ffi::GtkCellRenderer,
    wdgtptr: *mut ffi::GtkWidget,
    width: c_int,
    min_height_ptr: *mut c_int,
    nat_height_ptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    let (min_height, nat_height) =
        imp.preferred_height_for_width(wrap.unsafe_cast_ref(), &*widget, width);
    if !min_height_ptr.is_null() {
        *min_height_ptr = min_height;
    }
    if !nat_height_ptr.is_null() {
        *nat_height_ptr = nat_height;
    }
}

unsafe extern "C" fn cell_renderer_get_preferred_height<T: CellRendererImpl>(
    ptr: *mut ffi::GtkCellRenderer,
    wdgtptr: *mut ffi::GtkWidget,
    minptr: *mut c_int,
    natptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    let (min_size, nat_size) = imp.preferred_height(wrap.unsafe_cast_ref(), &*widget);
    if !minptr.is_null() {
        *minptr = min_size;
    }
    if !natptr.is_null() {
        *natptr = nat_size;
    }
}

unsafe extern "C" fn cell_renderer_get_preferred_width_for_height<T: CellRendererImpl>(
    ptr: *mut ffi::GtkCellRenderer,
    wdgtptr: *mut ffi::GtkWidget,
    height: c_int,
    min_width_ptr: *mut c_int,
    nat_width_ptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    let (min_width, nat_width) =
        imp.preferred_width_for_height(wrap.unsafe_cast_ref(), &*widget, height);
    if !min_width_ptr.is_null() {
        *min_width_ptr = min_width;
    }
    if !nat_width_ptr.is_null() {
        *nat_width_ptr = nat_width;
    }
}

unsafe extern "C" fn cell_renderer_get_preferred_width<T: CellRendererImpl>(
    ptr: *mut ffi::GtkCellRenderer,
    wdgtptr: *mut ffi::GtkWidget,
    minptr: *mut c_int,
    natptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    let (min_size, nat_size) = imp.preferred_width(wrap.unsafe_cast_ref(), &*widget);
    if !minptr.is_null() {
        *minptr = min_size;
    }
    if !natptr.is_null() {
        *natptr = nat_size;
    }
}

unsafe extern "C" fn cell_renderer_get_request_mode<T: CellRendererImpl>(
    ptr: *mut ffi::GtkCellRenderer,
) -> ffi::GtkSizeRequestMode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);

    imp.request_mode(wrap.unsafe_cast_ref()).into_glib()
}

unsafe extern "C" fn cell_renderer_snapshot<T: CellRendererImpl>(
    ptr: *mut ffi::GtkCellRenderer,
    snapshotptr: *mut ffi::GtkSnapshot,
    wdgtptr: *mut ffi::GtkWidget,
    bgptr: *const gdk::ffi::GdkRectangle,
    cellptr: *const gdk::ffi::GdkRectangle,
    flags: ffi::GtkCellRendererState,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);
    let snapshot: Borrowed<Snapshot> = from_glib_borrow(snapshotptr);

    imp.snapshot(
        wrap.unsafe_cast_ref(),
        &snapshot,
        &*widget,
        &from_glib_borrow(bgptr),
        &from_glib_borrow(cellptr),
        from_glib(flags),
    );
}

unsafe extern "C" fn cell_renderer_start_editing<T: CellRendererImpl>(
    ptr: *mut ffi::GtkCellRenderer,
    evtptr: *mut gdk::ffi::GdkEvent,
    wdgtptr: *mut ffi::GtkWidget,
    pathptr: *const c_char,
    bgptr: *const gdk::ffi::GdkRectangle,
    cellptr: *const gdk::ffi::GdkRectangle,
    flags: ffi::GtkCellRendererState,
) -> *mut ffi::GtkCellEditable {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);
    let evt: Borrowed<Option<gdk::Event>> = from_glib_borrow(evtptr);

    imp.start_editing(
        wrap.unsafe_cast_ref(),
        evt.as_ref().as_ref(),
        &*widget,
        &GString::from_glib_borrow(pathptr),
        &from_glib_borrow(bgptr),
        &from_glib_borrow(cellptr),
        from_glib(flags),
    )
    .to_glib_none()
    .0
}
