use rand::Rng;
use wgpu::util::DeviceExt;

use super::vertex::Vertex;

pub struct PolygonBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

pub struct Polygon {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

pub fn random_polygon(device: &wgpu::Device) -> PolygonBuffer {
    let mut polygon: Polygon = Polygon {
        vertices: vec![],
        indices: vec![0, 1, 4, 1, 2, 4, 2, 3, 4],
    };

    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        polygon.vertices.push(
            Vertex {
                position: [rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0],
                color: [rng.gen(), 0.0, rng.gen()],
            }, // A
        )
    }

    PolygonBuffer {
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
