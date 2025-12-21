use anyhow::{Context, Result};
use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3, Vec4};
use wgpu::SurfaceError;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::visuals::camera::Camera;
use crate::visuals::camera_controller::CameraController;
use crate::visuals::grid::{self, GridSize, LineVertex, Wall};
use crate::visuals::units::lego;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    fn update(&mut self, view_proj: Mat4) {
        self.view_proj = view_proj.to_cols_array_2d();
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
}

impl From<LineVertex> for Vertex {
    fn from(v: LineVertex) -> Self {
        Self { position: v.position }
    }
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

const WALL_ORDER: [Wall; 6] = [
    Wall::XMin,
    Wall::XMax,
    Wall::YMin,
    Wall::YMax,
    Wall::ZMin,
    Wall::ZMax,
];

fn wall_index(w: Wall) -> usize {
    match w {
        Wall::XMin => 0,
        Wall::XMax => 1,
        Wall::YMin => 2,
        Wall::YMax => 3,
        Wall::ZMin => 4,
        Wall::ZMax => 5,
    }
}

// Returns the three non-camera-facing walls (one for each axis).
fn back_walls(eye: Vec3, target: Vec3) -> [Wall; 3] {
    let wx = if eye.x < target.x { Wall::XMax } else { Wall::XMin };
    let wy = if eye.y < target.y { Wall::YMax } else { Wall::YMin };
    let wz = if eye.z < target.z { Wall::ZMax } else { Wall::ZMin };
    [wx, wy, wz]
}

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    size: (u32, u32),
    camera: Camera,

    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    camera_controller: CameraController,

    line_pipeline: wgpu::RenderPipeline,

    grid_size: GridSize,
    wall_grid_vbufs: Vec<wgpu::Buffer>,
    wall_grid_vcounts: [u32; 6],
}

impl Renderer {
    pub async fn new(window: &'static Window) -> Result<Self> {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window).context("create wgpu surface")?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .context("request wgpu adapter")?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("ssm device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .context("request wgpu device")?;

        let win_size = window.inner_size();
        let width = win_size.width.max(1);
        let height = win_size.height.max(1);

        let caps = surface.get_capabilities(&adapter);

        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let present_mode = if caps.present_modes.contains(&wgpu::PresentMode::Mailbox) {
            wgpu::PresentMode::Mailbox
        } else {
            wgpu::PresentMode::Fifo
        };

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let camera = Camera::new(Vec3::new(10.0, -16.0, 10.0), Vec3::ZERO);

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("camera bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: std::num::NonZeroU64::new(
                            std::mem::size_of::<CameraUniform>() as u64,
                        ),
                    },
                    count: None,
                }],
            });

        let camera_uniform = CameraUniform::new();

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera buffer"),
            contents: bytemuck::bytes_of(&camera_uniform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera bind group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let camera_controller = CameraController::new(20.0, Vec3::ZERO);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("line shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("axis.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("line pipeline layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let line_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("line pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let grid_size = GridSize { x: 5, y: 5, z: 15 };

        let mut wall_grid_vbufs = Vec::with_capacity(6);
        let mut wall_grid_vcounts = [0u32; 6];

        for (i, wall) in WALL_ORDER.iter().copied().enumerate() {
            let lines = grid::build_wall_grid(grid_size, lego::SCALE_NORMALIZED, &[wall]);
            let vertices: Vec<Vertex> = lines.into_iter().map(Vertex::from).collect();

            let vbuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("grid vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            wall_grid_vcounts[i] = vertices.len() as u32;
            wall_grid_vbufs.push(vbuf);
        }

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size: (width, height),
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_controller,
            line_pipeline,
            grid_size,
            wall_grid_vbufs,
            wall_grid_vcounts,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let width = width.max(1);
        let height = height.max(1);

        self.size = (width, height);
        self.config.width = width;
        self.config.height = height;

        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self) -> Result<()> {
        match self.render_inner() {
            Ok(()) => Ok(()),
            Err(SurfaceError::Lost) => {
                self.surface.configure(&self.device, &self.config);
                Ok(())
            }
            Err(SurfaceError::OutOfMemory) => Err(anyhow::anyhow!("wgpu out of memory")),
            Err(e) => Err(anyhow::anyhow!("wgpu surface error: {e}")),
        }
    }

    pub fn on_camera_drag(&mut self, dx: f32, dy: f32) {
        self.camera_controller.on_mouse_drag(dx, dy);
        self.camera_controller.apply(&mut self.camera);
    }

    pub fn on_camera_scroll(&mut self, delta: f32) {
        self.camera_controller.on_scroll(delta);
        self.camera_controller.apply(&mut self.camera);
    }

    pub fn pick_focus_point(&self, x: f32, y: f32) -> Option<Vec3> {
        let (w, h) = self.size;
        let ndc_x = (2.0 * x) / w as f32 - 1.0;
        let ndc_y = 1.0 - (2.0 * y) / h as f32;

        let inv_view_proj = self.camera.view_proj(w as f32 / h as f32).inverse();

        let near = inv_view_proj * Vec4::new(ndc_x, ndc_y, -1.0, 1.0);
        let far = inv_view_proj * Vec4::new(ndc_x, ndc_y, 1.0, 1.0);

        let near = (near / near.w).truncate();
        let far = (far / far.w).truncate();

        let dir = (far - near).normalize();

        let scale = lego::SCALE_NORMALIZED;
        let xmin = 0.0_f32;
        let ymin = 0.0_f32;
        let zmin = 0.0_f32;
        let xmax = self.grid_size.x as f32 * scale.xy;
        let ymax = self.grid_size.y as f32 * scale.xy;
        let zmax = self.grid_size.z as f32 * scale.z;

        let walls = back_walls(self.camera.eye, self.camera.target);

        intersect_walls(near, dir, &walls, xmin, xmax, ymin, ymax, zmin, zmax)
    }

    pub fn set_focus(&mut self, target: Vec3) {
        self.camera_controller.target = target;
        self.camera_controller.apply(&mut self.camera);
    }

    fn render_inner(&mut self) -> std::result::Result<(), SurfaceError> {
        let aspect = (self.size.0 as f32) / (self.size.1 as f32);
        let view_proj = self.camera.view_proj(aspect);

        self.camera_uniform.update(view_proj);
        self.queue
            .write_buffer(&self.camera_buffer, 0, bytemuck::bytes_of(&self.camera_uniform));

        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("ssm encoder"),
                });

        {
            let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ssm pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.06,
                            g: 0.08,
                            b: 0.14,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            rp.set_pipeline(&self.line_pipeline);
            rp.set_bind_group(0, &self.camera_bind_group, &[]);

            let walls = back_walls(self.camera.eye, self.camera.target);
            for wall in walls {
                let i = wall_index(wall);
                rp.set_vertex_buffer(0, self.wall_grid_vbufs[i].slice(..));
                rp.draw(0..self.wall_grid_vcounts[i], 0..1);
            }

            let _ = &self.grid_size;
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }
}

fn intersect_walls(
    origin: Vec3,
    dir: Vec3,
    walls: &[Wall],
    xmin: f32,
    xmax: f32,
    ymin: f32,
    ymax: f32,
    zmin: f32,
    zmax: f32,
) -> Option<Vec3> {
    let eps = 1e-5_f32;

    let mut best_t: Option<f32> = None;
    let mut best_p: Option<Vec3> = None;

    let mut consider = |t: f32, p: Vec3| {
        if t < 0.0 {
            return;
        }
        if let Some(bt) = best_t {
            if t >= bt {
                return;
            }
        }
        best_t = Some(t);
        best_p = Some(p);
    };

    for &wall in walls {
        match wall {
            Wall::XMin => {
                if dir.x.abs() <= eps {
                    continue;
                }
                let t = (xmin - origin.x) / dir.x;
                let p = origin + dir * t;
                if p.y >= ymin - eps && p.y <= ymax + eps && p.z >= zmin - eps && p.z <= zmax + eps
                {
                    consider(t, p);
                }
            }
            Wall::XMax => {
                if dir.x.abs() <= eps {
                    continue;
                }
                let t = (xmax - origin.x) / dir.x;
                let p = origin + dir * t;
                if p.y >= ymin - eps && p.y <= ymax + eps && p.z >= zmin - eps && p.z <= zmax + eps
                {
                    consider(t, p);
                }
            }
            Wall::YMin => {
                if dir.y.abs() <= eps {
                    continue;
                }
                let t = (ymin - origin.y) / dir.y;
                let p = origin + dir * t;
                if p.x >= xmin - eps && p.x <= xmax + eps && p.z >= zmin - eps && p.z <= zmax + eps
                {
                    consider(t, p);
                }
            }
            Wall::YMax => {
                if dir.y.abs() <= eps {
                    continue;
                }
                let t = (ymax - origin.y) / dir.y;
                let p = origin + dir * t;
                if p.x >= xmin - eps && p.x <= xmax + eps && p.z >= zmin - eps && p.z <= zmax + eps
                {
                    consider(t, p);
                }
            }
            Wall::ZMin => {
                if dir.z.abs() <= eps {
                    continue;
                }
                let t = (zmin - origin.z) / dir.z;
                let p = origin + dir * t;
                if p.x >= xmin - eps && p.x <= xmax + eps && p.y >= ymin - eps && p.y <= ymax + eps
                {
                    consider(t, p);
                }
            }
            Wall::ZMax => {
                if dir.z.abs() <= eps {
                    continue;
                }
                let t = (zmax - origin.z) / dir.z;
                let p = origin + dir * t;
                if p.x >= xmin - eps && p.x <= xmax + eps && p.y >= ymin - eps && p.y <= ymax + eps
                {
                    consider(t, p);
                }
            }
        }
    }

    best_p
}


#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    fn assert_vec3_close(a: Vec3, b: Vec3) {
        let eps = 1e-4_f32;
        assert!((a.x - b.x).abs() <= eps, "x mismatch: {} vs {}", a.x, b.x);
        assert!((a.y - b.y).abs() <= eps, "y mismatch: {} vs {}", a.y, b.y);
        assert!((a.z - b.z).abs() <= eps, "z mismatch: {} vs {}", a.z, b.z);
    }

    #[test]
    fn back_walls_select_non_camera_facing() {
        let target = Vec3::ZERO;

        // Camera on negative X, negative Y, positive Z side
        let eye = Vec3::new(-1.0, -1.0, 1.0);
        let walls = back_walls(eye, target);
        assert!(matches!(walls[0], Wall::XMax));
        assert!(matches!(walls[1], Wall::YMax));
        assert!(matches!(walls[2], Wall::ZMin));

        // Camera on positive X, positive Y, negative Z side
        let eye = Vec3::new(1.0, 1.0, -1.0);
        let walls = back_walls(eye, target);
        assert!(matches!(walls[0], Wall::XMin));
        assert!(matches!(walls[1], Wall::YMin));
        assert!(matches!(walls[2], Wall::ZMax));
    }

    #[test]
    fn intersect_walls_hits_floor_when_visible() {
        let origin = Vec3::new(0.5, 0.5, 2.0);
        let dir = Vec3::new(0.0, 0.0, -1.0);

        let p = intersect_walls(
            origin,
            dir,
            &[Wall::ZMin],
            0.0,
            1.0,
            0.0,
            1.0,
            0.0,
            1.0,
        )
        .expect("expected hit");

        assert_vec3_close(p, Vec3::new(0.5, 0.5, 0.0));
    }

    #[test]
    fn intersect_walls_ignores_non_visible_closest_wall() {
        // Ray enters the box through YMin (closest), then exits through YMax.
        // If only YMax is "visible", we should return the YMax hit.
        let origin = Vec3::new(0.5, -1.0, 0.5);
        let dir = Vec3::new(0.0, 1.0, 0.0);

        let p = intersect_walls(
            origin,
            dir,
            &[Wall::YMax],
            0.0,
            1.0,
            0.0,
            1.0,
            0.0,
            1.0,
        )
        .expect("expected hit");

        assert_vec3_close(p, Vec3::new(0.5, 1.0, 0.5));
    }

    #[test]
    fn intersect_walls_returns_none_when_no_hit() {
        let origin = Vec3::new(2.0, 2.0, 2.0);
        let dir = Vec3::new(1.0, 0.0, 0.0);

        let p = intersect_walls(
            origin,
            dir,
            &[Wall::XMin, Wall::XMax, Wall::YMin, Wall::YMax, Wall::ZMin, Wall::ZMax],
            0.0,
            1.0,
            0.0,
            1.0,
            0.0,
            1.0,
        );

        assert!(p.is_none());
    }
}
