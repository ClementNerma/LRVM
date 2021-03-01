/// VM runner configuration to use with 'run_vm' or 'exec_vm' from 'lrvm_tools::debug'
#[derive(Debug, Clone, Copy)]
pub struct RunConfig {
    pub cycles_limit: Option<u128>,
    pub halt_on_exception: bool,
    pub print_cycles: bool,
    pub print_exceptions: bool,
    pub print_finish: bool,
    pub newline_on_finish: bool,
}

impl RunConfig {
    /// Create a default runner configuration.
    /// Prints exceptions and the cycle and instruction address when the VM halts.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a "halt on exception" runner configuration.
    /// Prints exceptions and the cycle and instruction address when the VM halts, stops when an exception occurrs.
    pub fn halt_on_ex() -> Self {
        Self::new().with_halt_on_exception(true)
    }

    /// Create a verbose runner configuration.
    /// Prints a message on each CPU cycle, as well as exceptions and the cycle number and instruction address when the VM stops.
    pub fn verbose() -> Self {
        Self::new().be_verbose()
    }

    /// Create a quiet runner configuration.
    /// Displays nothing.
    pub fn quiet() -> Self {
        Self::new().be_quiet()
    }

    /// Set if the VM should be stopped after a given number of CPU cycles.
    pub fn with_cycles_limit(mut self, limit: Option<u128>) -> Self {
        self.cycles_limit = limit;
        self
    }

    /// Set if the VM should be stopped when an exception occurrs.
    /// Note that many kernels _use_ a system of exceptions to work, so enabling this is only adviced if you know your program should NEVER
    /// encounter an exception.
    pub fn with_halt_on_exception(mut self, halt: bool) -> Self {
        self.halt_on_exception = halt;
        self
    }

    /// Set if the runner should display a message on each CPU cycle.
    /// Only for debugging purpose, as LRVM usually runs several hundred thousand cycles per second (in debug mode).
    pub fn with_print_cycles(mut self, print: bool) -> Self {
        self.print_cycles = print;
        self
    }

    /// Set if the runner should display a warning when an exception occurrs.
    pub fn with_print_exceptions(mut self, print: bool) -> Self {
        self.print_exceptions = print;
        self
    }

    /// Set if the runner should print a message when the VM halts (cycle number, end address).
    pub fn with_print_finish(mut self, print: bool) -> Self {
        self.print_finish = print;
        self
    }

    /// Set if the runner should print a newline when the VM halts.
    pub fn with_newline_on_finish(mut self, print: bool) -> Self {
        self.newline_on_finish = print;
        self
    }

    /// Enable all display informations.
    pub fn be_verbose(mut self) -> Self {
        self.print_cycles = true;
        self.print_exceptions = true;
        self.print_finish = true;
        self
    }

    /// Suppress all displays.
    pub fn be_quiet(mut self) -> Self {
        self.print_cycles = false;
        self.print_exceptions = false;
        self.print_finish = false;
        self
    }
}

impl Default for RunConfig {
    fn default() -> Self {
        Self {
            cycles_limit: None,
            halt_on_exception: false,
            print_cycles: false,
            print_exceptions: true,
            print_finish: true,
            newline_on_finish: false,
        }
    }
}
