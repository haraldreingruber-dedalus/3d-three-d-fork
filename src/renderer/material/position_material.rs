use crate::core::*;
use crate::renderer::*;

///
/// Render the object with colors that reflect its position which primarily is used for debug purposes.
/// The x coordinate maps to the red channel, y to green and z to blue.
///
#[derive(Default, Clone)]
pub struct PositionMaterial {
    /// Render states.
    pub render_states: RenderStates,
}

impl FromCpuMaterial for PositionMaterial {
    fn from_cpu_material(_context: &Context, _cpu_material: &CpuMaterial) -> Self {
        Self::default()
    }
}

impl Material for PositionMaterial {
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        _lights: &[&dyn Light],
    ) -> Result<FragmentShader, RendererError> {
        let attributes = FragmentAttributes {
            position: true,
            ..FragmentAttributes::NONE
        };
        provided_attributes.ensure_contains_all(attributes)?;
        Ok(FragmentShader {
            source: include_str!("shaders/position_material.frag").to_string(),
            attributes,
        })
    }

    fn use_uniforms(&self, _program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {}

    fn render_states(&self) -> RenderStates {
        self.render_states
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
