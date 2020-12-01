use std::ptr;
use rand_core::{RngCore};

use super::Color;
use super::utils::rand_unit;

#[derive(Clone, Copy, Debug)]
pub struct PointData {
  pub thresh_band : f32,
  pub color_band : f32,
}

impl PointData {
  pub fn new(thresh_band : f32, color_band : f32) -> Self {
    PointData {
      thresh_band: thresh_band,
      color_band: color_band,
    }
  }

  pub fn sample(&self, rng: &mut dyn RngCore, thresh : f32, thresh_band : f32) -> Color {
    if (self.thresh_band - thresh).abs() < thresh_band {
      return Color::THRESH;
    }

    if self.thresh_band > thresh {
      let seed = rand_unit(rng);
      if self.color_band < seed {
        Color::X
      }
      else {
        Color::Y
      }
    }
    else {
      Color::Z
    }
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct DirtyRect {
    pub min_x : u32,
    pub max_x : u32,
    pub min_y : u32,
    pub max_y : u32,
}

impl DirtyRect {
    pub fn is_clean(&self) -> bool {
        self.max_x < self.min_x && self.max_y < self.min_y
    }

    pub fn clean_rect(width : u32, height : u32) -> Self {
        DirtyRect {
            min_x : width,
            min_y : height,
            max_x : 0,
            max_y : 0,
        }
    }

    pub fn all_dirty(width: u32, height : u32) -> Self {
        DirtyRect {
            min_x : 0,
            min_y : 0,
            max_x : width,
            max_y : height,
        }
    }
}

/// Contains a mutable and immutable copy of the same data.
/// 
/// We use this to provide an interface to brushes. The brush can
/// read data from the immutable buffer representing data in the previous frame,
/// and write to the mutable data to prepare the next frame.
pub struct PointDataStore {
    mut_data : Vec<PointData>,
    imm_data : Vec<PointData>,
    width : u32,
    height : u32,
    dirty : DirtyRect,
    dirty_last : DirtyRect,
}

impl PointDataStore {
    pub fn new(data : Vec<PointData>, width : u32, height : u32) -> Self {

        let dirty = DirtyRect::clean_rect(width as u32, height as u32);
        let dirty_last = dirty.clone();

        PointDataStore {
            mut_data: data.clone(),
            imm_data: data,
            width : width as u32,
            height : height as u32,
            dirty : dirty,
            dirty_last : dirty_last,
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn try_get_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            None
        }
        else {
            Some((y as u32 * self.width + x as u32) as usize)
        }
    }

    pub fn get_mut(&mut self, x : u32, y : u32) -> &mut PointData {
        debug_assert!(true);
        debug_assert!(false);
        let i = self.get_index(x, y);
        &mut self.mut_data[i]
    }

    pub fn get(&self, x : u32, y : u32) -> PointData {
        let i = self.get_index(x, y);
        self.imm_data[i]
    }

    pub fn try_get(&self, x : i32, y : i32) -> Option<PointData> {
        self.try_get_index(x, y)
            .map(|i| self.imm_data[i])
    }

    pub fn set_dirty(&mut self, rect : DirtyRect) {
        self.dirty = rect
    }

    #[allow(dead_code)]
    pub fn get_dirty(&self) -> &DirtyRect {
        &self.dirty
    }

    pub fn get_last_dirty(&self) -> &DirtyRect {
        &self.dirty_last
    }

    pub fn get_clone(&self) -> Vec<PointData> {
        self.mut_data.clone()
    }

    pub fn flip_hoz(&mut self) {
        self.dirty = DirtyRect::all_dirty(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.get_index(x, y);
                let isrc = self.get_index(self.width - x - 1, y);
                self.mut_data[i] = self.imm_data[isrc].clone();
            }
        }
    }

    /// Flush the mutable data and copy it into the immutable buffer.
    pub fn flush(&mut self) {
        // Only do work if there is a section that needs to be copied.
        if !self.dirty.is_clean() {
            unsafe {
                // Iterate over the section with changes calling memcpy on each line within the
                // changed region.
                let len = self.dirty.max_x - self.dirty.min_x;
                for y in self.dirty.min_y..self.dirty.max_y {
                    let offset = self.get_index(self.dirty.min_x, y);
                    let src_ptr = self.mut_data.as_ptr().wrapping_add(offset); 
                    let dst_ptr = self.imm_data.as_mut_ptr().wrapping_add(offset);

                    ptr::copy_nonoverlapping(src_ptr, dst_ptr, len as usize);
                }
            }
        }

        self.dirty_last = self.dirty;
        self.dirty = DirtyRect::clean_rect(self.width, self.height)
    }
}