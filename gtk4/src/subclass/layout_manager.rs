// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, Object};
use libc::c_int;

use crate::{LayoutChild, LayoutManager, Orientation, SizeRequestMode, Widget};

pub trait LayoutManagerImpl: LayoutManagerImplExt + ObjectImpl {
    fn allocate(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        width: i32,
        height: i32,
        baseline: i32,
    ) {
        self.parent_allocate(layout_manager, widget, width, height, baseline)
    }

    fn create_layout_child(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        for_child: &Widget,
    ) -> LayoutChild {
        self.parent_create_layout_child(layout_manager, widget, for_child)
    }

    /// Only set if the child implemented LayoutChildImpl
    fn layout_child_type() -> Option<glib::Type> {
        None
    }

    #[doc(alias = "get_request_mode")]
    fn request_mode(&self, layout_manager: &Self::Type, widget: &Widget) -> SizeRequestMode {
        self.parent_request_mode(layout_manager, widget)
    }

    fn measure(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        self.parent_measure(layout_manager, widget, orientation, for_size)
    }

    fn root(&self, layout_manager: &Self::Type) {
        self.parent_root(layout_manager)
    }

    fn unroot(&self, layout_manager: &Self::Type) {
        self.parent_unroot(layout_manager)
    }
}

pub trait LayoutManagerImplExt: ObjectSubclass {
    fn parent_allocate(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        width: i32,
        height: i32,
        baseline: i32,
    );

    fn parent_create_layout_child(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        for_child: &Widget,
    ) -> LayoutChild;

    fn parent_request_mode(&self, layout_manager: &Self::Type, widget: &Widget) -> SizeRequestMode;

    fn parent_measure(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32);

    fn parent_root(&self, layout_manager: &Self::Type);

    fn parent_unroot(&self, layout_manager: &Self::Type);
}

impl<T: LayoutManagerImpl> LayoutManagerImplExt for T {
    fn parent_allocate(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        width: i32,
        height: i32,
        baseline: i32,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkLayoutManagerClass;
            if let Some(f) = (*parent_class).allocate {
                f(
                    layout_manager
                        .unsafe_cast_ref::<LayoutManager>()
                        .to_glib_none()
                        .0,
                    widget.to_glib_none().0,
                    width,
                    height,
                    baseline,
                )
            }
        }
    }

    fn parent_create_layout_child(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        for_child: &Widget,
    ) -> LayoutChild {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkLayoutManagerClass;
            let f = (*parent_class)
                .create_layout_child
                .expect("No parent class impl for \"create_layout_child\"");
            from_glib_none(f(
                layout_manager
                    .unsafe_cast_ref::<LayoutManager>()
                    .to_glib_none()
                    .0,
                widget.to_glib_none().0,
                for_child.to_glib_none().0,
            ))
        }
    }

    fn parent_request_mode(&self, layout_manager: &Self::Type, widget: &Widget) -> SizeRequestMode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkLayoutManagerClass;
            let f = (*parent_class)
                .get_request_mode
                .expect("No parent class impl for \"get_request_mode\"");
            from_glib(f(
                layout_manager
                    .unsafe_cast_ref::<LayoutManager>()
                    .to_glib_none()
                    .0,
                widget.to_glib_none().0,
            ))
        }
    }

    fn parent_measure(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkLayoutManagerClass;
            let f = (*parent_class)
                .measure
                .expect("No parent class impl for \"measure\"");

            let mut minimum = 0;
            let mut natural = 0;
            let mut minimum_baseline = -1;
            let mut natural_baseline = -1;
            f(
                layout_manager
                    .unsafe_cast_ref::<LayoutManager>()
                    .to_glib_none()
                    .0,
                widget.to_glib_none().0,
                orientation.into_glib(),
                for_size,
                &mut minimum,
                &mut natural,
                &mut minimum_baseline,
                &mut natural_baseline,
            );
            (minimum, natural, minimum_baseline, natural_baseline)
        }
    }

    fn parent_root(&self, layout_manager: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkLayoutManagerClass;
            if let Some(f) = (*parent_class).root {
                f(layout_manager
                    .unsafe_cast_ref::<LayoutManager>()
                    .to_glib_none()
                    .0)
            }
        }
    }

    fn parent_unroot(&self, layout_manager: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkLayoutManagerClass;
            if let Some(f) = (*parent_class).unroot {
                f(layout_manager
                    .unsafe_cast_ref::<LayoutManager>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

unsafe impl<T: LayoutManagerImpl> IsSubclassable<T> for LayoutManager {
    fn class_init(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.allocate = Some(layout_manager_allocate::<T>);
        klass.create_layout_child = Some(layout_manager_create_layout_child::<T>);
        if let Some(type_) = T::layout_child_type() {
            klass.layout_child_type = type_.into_glib();
        }
        klass.get_request_mode = Some(layout_manager_get_request_mode::<T>);
        klass.measure = Some(layout_manager_measure::<T>);
        klass.root = Some(layout_manager_root::<T>);
        klass.unroot = Some(layout_manager_unroot::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn layout_manager_allocate<T: LayoutManagerImpl>(
    ptr: *mut ffi::GtkLayoutManager,
    widgetptr: *mut ffi::GtkWidget,
    width: i32,
    height: i32,
    baseline: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<LayoutManager> = from_glib_borrow(ptr);

    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);

    imp.allocate(wrap.unsafe_cast_ref(), &widget, width, height, baseline)
}

unsafe extern "C" fn layout_manager_create_layout_child<T: LayoutManagerImpl>(
    ptr: *mut ffi::GtkLayoutManager,
    widgetptr: *mut ffi::GtkWidget,
    for_childptr: *mut ffi::GtkWidget,
) -> *mut ffi::GtkLayoutChild {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<LayoutManager> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);
    let for_child: Borrowed<Widget> = from_glib_borrow(for_childptr);

    imp.create_layout_child(wrap.unsafe_cast_ref(), &widget, &for_child)
        .to_glib_full()
}

unsafe extern "C" fn layout_manager_get_request_mode<T: LayoutManagerImpl>(
    ptr: *mut ffi::GtkLayoutManager,
    widgetptr: *mut ffi::GtkWidget,
) -> ffi::GtkSizeRequestMode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<LayoutManager> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);

    imp.request_mode(wrap.unsafe_cast_ref(), &widget)
        .into_glib()
}

unsafe extern "C" fn layout_manager_measure<T: LayoutManagerImpl>(
    ptr: *mut ffi::GtkLayoutManager,
    widgetptr: *mut ffi::GtkWidget,
    orientation: ffi::GtkOrientation,
    for_size: i32,
    minimum_ptr: *mut c_int,
    natural_ptr: *mut c_int,
    minimum_baseline_ptr: *mut c_int,
    natural_baseline_ptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<LayoutManager> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);

    let (minimum, natural, minimum_baseline, natural_baseline) = imp.measure(
        wrap.unsafe_cast_ref(),
        &widget,
        from_glib(orientation),
        for_size,
    );
    if !minimum_ptr.is_null() {
        *minimum_ptr = minimum;
    }
    if !natural_ptr.is_null() {
        *natural_ptr = natural;
    }
    if !minimum_baseline_ptr.is_null() {
        *minimum_baseline_ptr = minimum_baseline;
    }
    if !natural_baseline_ptr.is_null() {
        *natural_baseline_ptr = natural_baseline;
    }
}

unsafe extern "C" fn layout_manager_root<T: LayoutManagerImpl>(ptr: *mut ffi::GtkLayoutManager) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<LayoutManager> = from_glib_borrow(ptr);

    imp.root(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn layout_manager_unroot<T: LayoutManagerImpl>(ptr: *mut ffi::GtkLayoutManager) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<LayoutManager> = from_glib_borrow(ptr);

    imp.unroot(wrap.unsafe_cast_ref())
}
