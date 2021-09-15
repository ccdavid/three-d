//!
//! A collection of objects that can be rendered, for example a mesh.
//!

pub use crate::core::{AxisAlignedBoundingBox, CPUMesh, Cull};

mod model;
#[doc(inline)]
pub use model::*;

mod model2d;
#[doc(inline)]
pub use model2d::*;

mod instanced_model;
#[doc(inline)]
pub use instanced_model::*;

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

use crate::core::*;
use crate::renderer::*;

pub trait Object: Geometry {
    fn render(
        &self,
        paint: &dyn Paint,
        camera: &Camera,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()>;

    fn render_deferred(&self, paint: &dyn Paint, camera: &Camera, viewport: Viewport)
        -> Result<()>;
}

pub trait Geometry {
    ///
    /// Render only the depth into the current depth render target which is useful for shadow maps or depth pre-pass.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render_depth(&self, camera: &Camera) -> Result<()>;

    ///
    /// Render the depth (scaled such that a value of 1 corresponds to max_depth) into the red channel of the current color render target which for example is used for picking.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render_depth_to_red(&self, camera: &Camera, max_depth: f32) -> Result<()>;

    fn aabb(&self) -> AxisAlignedBoundingBox;
}
