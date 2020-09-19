use elektrovulkan::{Context, ProgramProc};

fn main() {
    let (sld_context, program_proc) = ProgramProc::new();
    let vulkan_app = Context::new(&program_proc.event_pump, sld_context);

    program_proc.main_loop(vulkan_app);
}