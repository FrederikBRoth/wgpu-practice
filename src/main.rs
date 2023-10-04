use wgpu_practice::ui::game_loop::run;
fn main() {
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    pollster::block_on(run());
}
