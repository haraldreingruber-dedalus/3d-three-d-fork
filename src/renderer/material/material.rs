use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

///
/// A physically-based material used for shading an object.
///
#[derive(Clone)]
pub struct Material {
    /// Name. Used for matching geometry and material.
    pub name: String,
    /// Albedo base color, also called diffuse color.
    pub albedo: Vec4,
    /// Texture with albedo base colors, also called diffuse color.
    pub albedo_texture: Option<Rc<Texture2D>>,
    /// A value in the range `[0..1]` specifying how metallic the material is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the material surface is.
    pub roughness: f32,
    /// Texture containing the metallic and roughness parameters which are multiplied with the [Self::metallic] and [Self::roughness] values in the shader.
    /// The metallic values are sampled from the blue channel and the roughness from the green channel.
    pub metallic_roughness_texture: Option<Rc<Texture2D>>,
    /// A scalar multiplier controlling the amount of occlusion applied from the [Self::occlusion_texture]. A value of 0.0 means no occlusion. A value of 1.0 means full occlusion.
    pub occlusion_strength: f32,
    /// An occlusion map. Higher values indicate areas that should receive full indirect lighting and lower values indicate no indirect lighting.
    /// The occlusion values are sampled from the red channel.
    pub occlusion_texture: Option<Rc<Texture2D>>,
    /// A scalar multiplier applied to each normal vector of the [Self::normal_texture].
    pub normal_scale: f32,
    /// A tangent space normal map, also known as bump map.
    pub normal_texture: Option<Rc<Texture2D>>,
    pub lighting_model: LightingModel,
}

impl Material {
    ///
    /// Moves the material information from the [CPUMaterial] to the GPU.
    /// If the input contains an [CPUMaterial::occlusion_metallic_roughness_texture], this texture is used for both
    /// [Material::metallic_roughness_texture] and [Material::occlusion_texture] while any [CPUMaterial::metallic_roughness_texture] or [CPUMaterial::occlusion_texture] are ignored.
    ///
    pub fn new(context: &Context, cpu_material: &CPUMaterial) -> Result<Self> {
        let albedo_texture = if let Some(ref cpu_texture) = cpu_material.albedo_texture {
            Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
        } else {
            None
        };
        let metallic_roughness_texture =
            if let Some(ref cpu_texture) = cpu_material.occlusion_metallic_roughness_texture {
                Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
            } else {
                if let Some(ref cpu_texture) = cpu_material.metallic_roughness_texture {
                    Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
                } else {
                    None
                }
            };
        let occlusion_texture = if cpu_material.occlusion_metallic_roughness_texture.is_some() {
            metallic_roughness_texture.clone()
        } else {
            if let Some(ref cpu_texture) = cpu_material.occlusion_texture {
                Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
            } else {
                None
            }
        };
        let normal_texture = if let Some(ref cpu_texture) = cpu_material.normal_texture {
            Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
        } else {
            None
        };
        Ok(Self {
            name: cpu_material.name.clone(),
            albedo: cpu_material.albedo.to_vec4(),
            albedo_texture,
            metallic: cpu_material.metallic,
            roughness: cpu_material.roughness,
            metallic_roughness_texture,
            normal_texture,
            normal_scale: cpu_material.normal_scale,
            occlusion_texture,
            occlusion_strength: cpu_material.occlusion_strength,
            lighting_model: LightingModel::Blinn,
        })
    }

    fn bind_internal(&self, program: &Program) -> Result<()> {
        program.use_uniform_float("metallic", &self.metallic)?;
        program.use_uniform_float("roughness", &self.roughness)?;
        program.use_uniform_vec4("albedo", &self.albedo)?;
        if let Some(ref texture) = self.albedo_texture {
            program.use_texture("albedoTexture", texture.as_ref())?;
        }
        if let Some(ref texture) = self.metallic_roughness_texture {
            program.use_texture("metallicRoughnessTexture", texture.as_ref())?;
        }
        if let Some(ref texture) = self.occlusion_texture {
            program.use_uniform_float("occlusionStrength", &self.occlusion_strength)?;
            program.use_texture("occlusionTexture", texture.as_ref())?;
        }
        if let Some(ref texture) = self.normal_texture {
            program.use_uniform_float("normalScale", &self.normal_scale)?;
            program.use_texture("normalTexture", texture.as_ref())?;
        }
        Ok(())
    }
}

impl Paint for Material {
    fn fragment_shader_source(
        &self,
        _ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> String {
        shaded_fragment_shader(
            self.lighting_model,
            Some(self),
            directional_lights.len(),
            spot_lights.len(),
            point_lights.len(),
        )
    }
    fn bind(
        &self,
        program: &Program,
        camera: &Camera,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()> {
        bind_lights(
            program,
            ambient_light,
            directional_lights,
            spot_lights,
            point_lights,
            camera.position(),
        )?;
        self.bind_internal(program)
    }

    fn render_states(&self) -> RenderStates {
        let transparent = self.albedo[3] < 0.99
            || self
                .albedo_texture
                .as_ref()
                .map(|t| t.is_transparent())
                .unwrap_or(false);

        if transparent {
            RenderStates {
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            }
        } else {
            RenderStates::default()
        }
    }
}

impl DeferredMaterial for Material {
    fn fragment_shader_source(&self) -> String {
        format!(
            "in vec3 pos;\nin vec3 nor;\n{}{}",
            material_shader(self),
            include_str!("shaders/deferred_objects.frag")
        )
    }
    fn bind(&self, program: &Program) -> Result<()> {
        self.bind_internal(program)
    }

    fn render_states(&self) -> RenderStates {
        RenderStates::default()
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            albedo: vec4(1.0, 1.0, 1.0, 1.0),
            albedo_texture: None,
            metallic: 0.0,
            roughness: 1.0,
            metallic_roughness_texture: None,
            normal_texture: None,
            normal_scale: 1.0,
            occlusion_texture: None,
            occlusion_strength: 1.0,
            lighting_model: LightingModel::Blinn,
        }
    }
}

pub(in crate::renderer) fn bind_lights(
    program: &Program,
    ambient_light: Option<&AmbientLight>,
    directional_lights: &[&DirectionalLight],
    spot_lights: &[&SpotLight],
    point_lights: &[&PointLight],
    camera_position: &Vec3,
) -> Result<()> {
    // Ambient light
    program.use_uniform_vec3(
        "ambientColor",
        &ambient_light
            .map(|light| light.color.to_vec3() * light.intensity)
            .unwrap_or(vec3(0.0, 0.0, 0.0)),
    )?;

    if !directional_lights.is_empty() || !spot_lights.is_empty() || !point_lights.is_empty() {
        program.use_uniform_vec3("eyePosition", camera_position)?;
    }

    // Directional light
    for i in 0..directional_lights.len() {
        program.use_texture(
            &format!("directionalShadowMap{}", i),
            directional_lights[i].shadow_map(),
        )?;
        program.use_uniform_block(
            &format!("DirectionalLightUniform{}", i),
            directional_lights[i].buffer(),
        );
    }

    // Spot light
    for i in 0..spot_lights.len() {
        program.use_texture(&format!("spotShadowMap{}", i), spot_lights[i].shadow_map())?;
        program.use_uniform_block(&format!("SpotLightUniform{}", i), spot_lights[i].buffer());
    }

    // Point light
    for i in 0..point_lights.len() {
        program.use_uniform_block(&format!("PointLightUniform{}", i), point_lights[i].buffer());
    }
    Ok(())
}

pub(in crate::renderer) fn shaded_fragment_shader(
    lighting_model: LightingModel,
    material: Option<&Material>,
    directional_lights: usize,
    spot_lights: usize,
    point_lights: usize,
) -> String {
    let mut dir_uniform = String::new();
    let mut dir_fun = String::new();
    for i in 0..directional_lights {
        dir_uniform.push_str(&format!(
            "
                uniform sampler2D directionalShadowMap{};
                layout (std140) uniform DirectionalLightUniform{}
                {{
                    DirectionalLight directionalLight{};
                }};",
            i, i, i
        ));
        dir_fun.push_str(&format!("
                    color += calculate_directional_light(directionalLight{}, surface_color, position, normal, metallic, roughness, occlusion, directionalShadowMap{});", i, i));
    }
    let mut spot_uniform = String::new();
    let mut spot_fun = String::new();
    for i in 0..spot_lights {
        spot_uniform.push_str(&format!(
            "
                uniform sampler2D spotShadowMap{};
                layout (std140) uniform SpotLightUniform{}
                {{
                    SpotLight spotLight{};
                }};",
            i, i, i
        ));
        spot_fun.push_str(&format!(
            "
                    color += calculate_spot_light(spotLight{}, surface_color, position, normal, metallic, roughness, occlusion, spotShadowMap{});",
            i, i
        ));
    }
    let mut point_uniform = String::new();
    let mut point_fun = String::new();
    for i in 0..point_lights {
        point_uniform.push_str(&format!(
            "
                layout (std140) uniform PointLightUniform{}
                {{
                    PointLight pointLight{};
                }};",
            i, i
        ));
        point_fun.push_str(&format!(
            "
                    color += calculate_point_light(pointLight{}, surface_color, position, normal, metallic, roughness, occlusion);",
            i
        ));
    }

    let model = match lighting_model {
        LightingModel::Phong => "#define PHONG",
        LightingModel::Blinn => "#define BLINN",
        LightingModel::Cook(normal, _) => match normal {
            NormalDistributionFunction::Blinn => "#define COOK\n#define COOK_BLINN\n",
            NormalDistributionFunction::Beckmann => "#define COOK\n#define COOK_BECKMANN\n",
            NormalDistributionFunction::TrowbridgeReitzGGX => "#define COOK\n#define COOK_GGX\n",
        },
    };

    format!(
        "{}\n{}\n{}\n{}\nin vec3 pos;\nin vec3 nor;\n{}\n{}",
        model,
        include_str!("../../core/shared.frag"),
        include_str!("shaders/light_shared.frag"),
        &format!(
            "
                uniform vec3 ambientColor;
                {} // Directional lights
                {} // Spot lights
                {} // Point lights

                vec3 calculate_lighting(vec3 surface_color, vec3 position, vec3 normal, float metallic, float roughness, float occlusion)
                {{
                    vec3 color = occlusion * ambientColor * mix(surface_color, vec3(0.0), metallic); // Ambient light
                    {} // Directional lights
                    {} // Spot lights
                    {} // Point lights
                    return color;
                }}
                ",
            &dir_uniform, &spot_uniform, &point_uniform, &dir_fun, &spot_fun, &point_fun
        ),
        material.map(|m| material_shader(m)).unwrap_or("#define DEFERRED\nin vec2 uv;\n".to_string()),
        include_str!("shaders/lighting.frag"),
    )
}

fn material_shader(material: &Material) -> String {
    let mut output = String::new();
    if material.albedo_texture.is_some()
        || material.metallic_roughness_texture.is_some()
        || material.normal_texture.is_some()
        || material.occlusion_texture.is_some()
    {
        output.push_str("in vec2 uvs;\n");
        if material.albedo_texture.is_some() {
            output.push_str("#define USE_ALBEDO_TEXTURE;\n");
        }
        if material.metallic_roughness_texture.is_some() {
            output.push_str("#define USE_METALLIC_ROUGHNESS_TEXTURE;\n");
        }
        if material.occlusion_texture.is_some() {
            output.push_str("#define USE_OCCLUSION_TEXTURE;\n");
        }
        if material.normal_texture.is_some() {
            output.push_str("#define USE_NORMAL_TEXTURE;\n");
        }
    }
    output
}