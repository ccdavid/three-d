//!
//! A collection of objects implementing the [Object] trait.
//!
//! Objects can be rendered directly or used in a render call, for example [RenderTarget::render].
//! Use the [Gm] struct to combine any [geometry] and [material] into an [Object].
//!

mod gm;
#[doc(inline)]
pub use gm::*;

mod model;
#[doc(inline)]
pub use model::*;

mod instanced_model;
#[doc(inline)]
pub use instanced_model::*;

mod voxel_grid;
#[doc(inline)]
pub use voxel_grid::*;

mod line;
#[doc(inline)]
pub use line::*;

mod rectangle;
#[doc(inline)]
pub use rectangle::*;

mod circle;
#[doc(inline)]
pub use circle::*;

mod skybox;
#[doc(inline)]
pub use skybox::*;

mod imposters;
#[doc(inline)]
pub use imposters::*;

mod axes;
#[doc(inline)]
pub use axes::*;

mod bounding_box;
#[doc(inline)]
pub use bounding_box::*;

use crate::core::*;
use crate::renderer::*;

///
/// Returns a camera for viewing 2D content.
///
pub fn camera2d(viewport: Viewport) -> Camera {
    Camera::new_orthographic(
        viewport,
        vec3(
            viewport.width as f32 * 0.5,
            viewport.height as f32 * 0.5,
            -1.0,
        ),
        vec3(
            viewport.width as f32 * 0.5,
            viewport.height as f32 * 0.5,
            0.0,
        ),
        vec3(0.0, -1.0, 0.0),
        viewport.height as f32,
        0.0,
        10.0,
    )
}

///
/// Represents a 3D object which can be rendered directly or used in a render call, for example [RenderTarget::render].
///
pub trait Object: Geometry {
    ///
    /// Render the object.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    fn render(&self, camera: &Camera, lights: &[&dyn Light]);

    ///
    /// Returns the type of material applied to this object.
    ///
    fn material_type(&self) -> MaterialType;
}

impl<T: Object + ?Sized> Object for &T {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) {
        (*self).render(camera, lights)
    }

    fn material_type(&self) -> MaterialType {
        (*self).material_type()
    }
}

impl<T: Object + ?Sized> Object for &mut T {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) {
        (**self).render(camera, lights)
    }

    fn material_type(&self) -> MaterialType {
        (**self).material_type()
    }
}

impl<T: Object> Object for Box<T> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) {
        self.as_ref().render(camera, lights)
    }

    fn material_type(&self) -> MaterialType {
        self.as_ref().material_type()
    }
}

impl<T: Object> Object for std::rc::Rc<T> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) {
        self.as_ref().render(camera, lights)
    }

    fn material_type(&self) -> MaterialType {
        self.as_ref().material_type()
    }
}

impl<T: Object> Object for std::sync::Arc<T> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) {
        self.as_ref().render(camera, lights)
    }

    fn material_type(&self) -> MaterialType {
        self.as_ref().material_type()
    }
}

impl<T: Object> Object for std::cell::RefCell<T> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) {
        self.borrow().render(camera, lights)
    }

    fn material_type(&self) -> MaterialType {
        self.borrow().material_type()
    }
}

impl<T: Object> Object for std::sync::RwLock<T> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) {
        self.read().unwrap().render(camera, lights)
    }

    fn material_type(&self) -> MaterialType {
        self.read().unwrap().material_type()
    }
}

// Object2D trait

///
/// Represents a 2D object which can be rendered.
///
pub trait Object2D: Geometry2D {
    ///
    /// Render the object.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    fn render(&self, viewport: Viewport);

    ///
    /// Returns the type of material applied to this object.
    ///
    fn material_type(&self) -> MaterialType;
}

impl<T: Object2D + ?Sized> Object2D for &T {
    fn render(&self, viewport: Viewport) {
        (*self).render(viewport)
    }

    fn material_type(&self) -> MaterialType {
        (*self).material_type()
    }
}

impl<T: Object2D + ?Sized> Object2D for &mut T {
    fn render(&self, viewport: Viewport) {
        (**self).render(viewport)
    }

    fn material_type(&self) -> MaterialType {
        (**self).material_type()
    }
}

impl<T: Object2D> Object2D for Box<T> {
    fn render(&self, viewport: Viewport) {
        self.as_ref().render(viewport)
    }

    fn material_type(&self) -> MaterialType {
        self.as_ref().material_type()
    }
}

impl<T: Object2D> Object2D for std::rc::Rc<T> {
    fn render(&self, viewport: Viewport) {
        self.as_ref().render(viewport)
    }

    fn material_type(&self) -> MaterialType {
        self.as_ref().material_type()
    }
}

impl<T: Object2D> Object2D for std::sync::Arc<T> {
    fn render(&self, viewport: Viewport) {
        self.as_ref().render(viewport)
    }

    fn material_type(&self) -> MaterialType {
        self.as_ref().material_type()
    }
}

impl<T: Object2D> Object2D for std::cell::RefCell<T> {
    fn render(&self, viewport: Viewport) {
        self.borrow().render(viewport)
    }

    fn material_type(&self) -> MaterialType {
        self.borrow().material_type()
    }
}

impl<T: Object2D> Object2D for std::sync::RwLock<T> {
    fn render(&self, viewport: Viewport) {
        self.read().unwrap().render(viewport)
    }

    fn material_type(&self) -> MaterialType {
        self.read().unwrap().material_type()
    }
}
