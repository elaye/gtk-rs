// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::cell_renderer::CellRendererImpl;
use crate::CellRenderer;
use crate::CellRendererSpinner;

pub trait CellRendererSpinnerImpl: CellRendererImpl {}

unsafe impl<T: CellRendererSpinnerImpl> IsSubclassable<T> for CellRendererSpinner {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <CellRenderer as IsSubclassable<T>>::override_vfuncs(class);
    }
}
