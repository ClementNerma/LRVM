use std::sync::{Arc, Mutex, MutexGuard};
use crate::cpu::CPU;
use super::{Bus, MappedMemory};

pub struct MotherBoard {
    aux: Vec<Arc<Mutex<Box<dyn Bus>>>>,
    mem: Arc<Mutex<MappedMemory>>,
    cpu: CPU
}

impl MotherBoard {
    pub fn new(components: impl IntoIterator<Item=Box<dyn Bus>>) -> Self {
        let (mut aux, mut mem_aux) = (vec![], vec![]);

        for component in components.into_iter() {
            let component = Arc::new(Mutex::new(component));
            aux.push(Arc::clone(&component));
            mem_aux.push(Arc::clone(&component));
        }

        let mem = Arc::new(Mutex::new(MappedMemory::new(mem_aux)));

        Self {
            aux,
            mem: Arc::clone(&mem),
            cpu: CPU::new(Arc::clone(&mem))
        }
    }

    pub fn map<T>(&mut self, mut mapper: impl FnMut(MutexGuard<MappedMemory>) -> T) -> T {
        mapper(self.mem.lock().unwrap())
    }

    pub fn cpu(&mut self) -> &mut CPU {
        &mut self.cpu
    }

    pub fn reset(&mut self) {
        self.cpu.reset();

        for aux in self.aux.iter() {
            aux.lock().unwrap().reset();
        }
    }
}
