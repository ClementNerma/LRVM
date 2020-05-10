use mrvm::board::Bus;
use super::{prepare_vm, run_vm, RunConfig, StoppedState};

/// Prepare a virtual machine with the provided components and run it with the provided configuration
pub fn exec_vm(components: Vec<Box<dyn Bus>>, config: &RunConfig) -> StoppedState {
    run_vm(prepare_vm(components).cpu(), config)
}
