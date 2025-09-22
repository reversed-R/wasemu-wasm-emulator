fn main() {
    let mut emulator = wasemu::emulator::builder::EmulatorBuilder::load("wasm code".as_bytes())
        .expect("failed to load")
        .link("print_i32", wasemu::emulator::builder::Linkable::Function)
        .expect("failed to link")
        .link(
            "awesome_complex_calculate",
            wasemu::emulator::builder::Linkable::Function,
        )
        .expect("failed to link")
        .link(
            "__linear_memory",
            wasemu::emulator::builder::Linkable::Memory,
        )
        .expect("failed to link")
        .build()
        .expect("failed to build");

    // in the case wasm contains start and run to the end
    emulator.start().expect("failed to run");

    // in the case wasm contains start and run with interval
    let mut emulator = emulator.start_until_limit(100).expect("failed to run");
    for _ in 0..10 {
        emulator = emulator.restart_until_limit(100).expect("failed to run");
    }
}
