use lrvm::board::{Bus, MotherBoard};

use super::{prepare_vm, run_vm, RunConfig, StoppedState};

/// Prepare a virtual machine with the provided components and run it with the provided configuration
pub fn exec_vm(components: Vec<Box<dyn Bus>>, config: RunConfig) -> (MotherBoard, StoppedState) {
    let mut motherboard = prepare_vm(components);
    let status = run_vm(motherboard.cpu(), config);
    (motherboard, status)
}
