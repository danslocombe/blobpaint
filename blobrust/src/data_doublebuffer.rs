use std::ptr;

// Represents a data double buffer where the current
pub struct DataDoubleBuffer<T> where T : Copy + Sized {
    mut_data : Vec<T>,
    imm_data : Vec<T>,
}

impl<T> DataDoubleBuffer<T> where T : Copy + Sized {
    pub fn new(data : Vec<T>) -> Self {
        DataDoubleBuffer {
            mut_data: data.clone(),
            imm_data: data,
        }
    }

    pub fn get_mut(&mut self) -> &mut [T] {
        &mut self.mut_data
    }

    pub fn get_imm(&self) -> &[T] {
        &self.imm_data
    }

    pub fn get_clone(&self) -> Vec<T> {
        self.mut_data.clone()
    }

    pub fn incr(&mut self) {
        unsafe {
            let src_ptr = self.mut_data.as_ptr(); 
            let dst_ptr = self.imm_data.as_mut_ptr();

            ptr::copy_nonoverlapping(src_ptr, dst_ptr, self.mut_data.len());
        }
    }
}