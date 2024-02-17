use crate::project::Project;
use web_sys::HtmlCanvasElement;
use webgl_matrix::Mat3;

pub trait Render<T, V>
where
    Self: Sized,
{
    fn new(project: &Project, canvas: HtmlCanvasElement) -> Result<Self, T>;
    fn render(&self, transformation: &Mat3) -> Result<(), V>;
}

mod webgl;

pub use webgl::WebGLMapRenderer;
