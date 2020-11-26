use std::ptr;

/// Contains a mutable and immutable copy of the same data.
/// 
/// We use this to provide an interface to brushes. The brush can
/// read data from the immutable buffer representing data in the previous frame,
/// and write to the mutable data to prepare the next frame.
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

    pub fn len(&self) -> usize {
        self.mut_data.len()
    }

    /// Flush the mutable data and copy it into the immutable buffer.
    pub fn flush(&mut self) {
        unsafe {
            let src_ptr = self.mut_data.as_ptr(); 
            let dst_ptr = self.imm_data.as_mut_ptr();

            ptr::copy_nonoverlapping(src_ptr, dst_ptr, self.mut_data.len());
        }
    }
}