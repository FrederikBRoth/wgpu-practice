use rand::Rng;
use wgpu::util::DeviceExt;

use super::vertex::Vertex;

pub struct MeshBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

pub fn random_polygon(device: &wgpu::Device) -> MeshBuffer {
    let mut polygon: Mesh = Mesh {
        vertices: vec![],
        indices: vec![0, 1, 4, 1, 2, 4, 2, 3, 4],
    };

    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        polygon.vertices.push(
            Vertex {
                position: [rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0],
                tex_coords: [rng.gen(), rng.gen()],
            }, // A
        )
    }

    MeshBuffer {
        vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&polygon.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }),
        index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&polygon.indices),
            usage: wgpu::BufferUsages::INDEX,
        }),
        num_indices: polygon.indices.len() as u32,
    }
}
pub fn make_pentagon(device: &wgpu::Device) -> MeshBuffer {
    let mut polygon: Mesh = Mesh {
        vertices: vec![],
        indices: vec![0, 1, 4, 1, 2, 4, 2, 3, 4],
    };

    let mut rng = rand::thread_rng();
    polygon.vertices.append(&mut vec![
        Vertex {
            position: [
                -0.0868241 + rng.gen_range(-0.2..0.2),
                0.49240386 + rng.gen_range(-0.2..0.2),
                0.0,
            ],
            tex_coords: [0.4131759, 1.0 - 0.99240386],
        }, // A
        Vertex {
            position: [
                -0.49513406 + rng.gen_range(-0.2..0.2),
                0.06958647 + rng.gen_range(-0.2..0.2),
                0.0,
            ],
            tex_coords: [0.0048659444, 1.0 - 0.56958647],
        }, // B
        Vertex {
            position: [
                -0.21918549 + rng.gen_range(-0.2..0.2),
                -0.44939706 + rng.gen_range(-0.2..0.2),
                0.0,
            ],
            tex_coords: [0.28081453, 1.0 - 0.05060294],
        }, // C
        Vertex {
            position: [
                0.35966998 + rng.gen_range(-0.2..0.2),
                -0.3473291 + rng.gen_range(-0.2..0.2),
                0.0,
            ],
            tex_coords: [0.85967, 1.0 - 0.1526709],
        }, // D
        Vertex {
            position: [
                0.44147372 + rng.gen_range(-0.2..0.2),
                0.2347359 + rng.gen_range(-0.2..0.2),
                0.0,
            ],
            tex_coords: [0.9414737, 1.0 - 0.7347359],
        }, // E
    ]);

    MeshBuffer {
        vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&polygon.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }),
        index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&polygon.indices),
            usage: wgpu::BufferUsages::INDEX,
        }),
        num_indices: polygon.indices.len() as u32,
    }
}
