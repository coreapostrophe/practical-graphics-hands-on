mod common;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    window.set_title("ch02_triangle_vertex_color");
    env_logger::init();
    let inputs = common::Inputs {
        source: wgpu::ShaderSource::Wgsl(include_str!("triangle_vertex_color.wgsl").into()),
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
    };
    pollster::block_on(common::run(event_loop, window, inputs, 3));
}
