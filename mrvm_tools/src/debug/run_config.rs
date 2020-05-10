
/// VM runner configuration
#[derive(Debug, Clone)]
pub struct RunConfig {
    pub cycles_limit: Option<u32>,
    pub halt_on_exception: bool,
    pub print_cycles: bool,
    pub print_exceptions: bool,
    pub print_finish: bool
}

impl RunConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ex_halt() -> Self {
        Self::new().with_halt_on_exception(true)
    }

    pub fn verbose() -> Self {
        Self::new().be_verbose()
    }

    pub fn quiet() -> Self {
        Self::new().be_quiet()
    }

    pub fn with_cycles_limit(mut self, limit: Option<u32>) -> Self {
        self.cycles_limit = limit;
        self
    }

    pub fn with_halt_on_exception(mut self, halt: bool) -> Self {
        self.halt_on_exception = halt;
        self
    }

    pub fn with_print_cycles(mut self, print: bool) -> Self {
        self.print_cycles = print;
        self
    }

    pub fn with_print_exceptions(mut self, print: bool) -> Self {
        self.print_exceptions = print;
        self
    }

    pub fn with_print_finish(mut self, print: bool) -> Self {
        self.print_finish = print;
        self
    }

    pub fn be_verbose(mut self) -> Self {
        self.print_cycles = true;
        self.print_finish = true;
        self
    }

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
            print_finish: true
        }
    }
}
