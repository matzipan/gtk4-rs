// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::{Border, Scrollable};
use glib::translate::*;
use glib::Cast;

pub trait ScrollableImpl: ObjectImpl {
    #[doc(alias = "get_border")]
    fn border(&self, scrollable: &Self::Type) -> Option<Border> {
        self.parent_border(scrollable)
    }
}

pub trait ScrollableImplExt: ObjectSubclass {
    fn parent_border(&self, scrollable: &Self::Type) -> Option<Border>;
}

impl<T: ScrollableImpl> ScrollableImplExt for T {
    fn parent_border(&self, scrollable: &Self::Type) -> Option<Border> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Scrollable>()
                as *const ffi::GtkScrollableInterface;

            if let Some(func) = (*parent_iface).get_border {
                let border = std::ptr::null_mut();
                if from_glib(func(
                    scrollable.unsafe_cast_ref::<Scrollable>().to_glib_none().0,
                    border,
                )) {
                    return Some(from_glib_none(border));
                }
            }
            None
        }
    }
}

unsafe impl<T: ScrollableImpl> IsImplementable<T> for Scrollable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.get_border = Some(scrollable_get_border::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn scrollable_get_border<T: ScrollableImpl>(
    scrollable: *mut ffi::GtkScrollable,
    borderptr: *mut ffi::GtkBorder,
) -> glib::ffi::gboolean {
    let instance = &*(scrollable as *mut T::Instance);
    let imp = instance.impl_();

    if let Some(border) =
        imp.border(from_glib_borrow::<_, Scrollable>(scrollable).unsafe_cast_ref())
    {
        *borderptr = *border.to_glib_full();
        true.into_glib()
    } else {
        *borderptr = ffi::GtkBorder {
            top: 0,
            right: 0,
            left: 0,
            bottom: 0,
        };
        false.into_glib()
    }
}
