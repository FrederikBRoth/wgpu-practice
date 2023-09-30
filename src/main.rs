use wgpu_practice::ui::game_loop::run;
fn main() {
    pollster::block_on(run());
}
