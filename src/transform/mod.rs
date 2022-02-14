#[derive(Default)]
pub struct Resize {
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub max_width: Option<usize>,
    pub max_height: Option<usize>,
    pub scale: Option<f32>,
}


impl Resize {
    pub fn calculate_dimensions(&self, current_width: usize, current_height: usize) -> (usize, usize) {
        let mut width = current_width as f32;
        let mut height = current_height as f32;
        if let Some(scale) = self.scale {
            width *= scale;
            height *= scale;
        }

        if let Some(max_width) = self.max_width {
            let max_width = max_width as f32;
            if width > max_width {
                height *= max_width / width;
                width = max_width;
            }
        }

        if let Some(max_height) = self.max_height {
            let max_height = max_height as f32;
            if height > max_height {
                width *= max_height / height;
                height = max_height;
            }
        }

        if self.width.is_some() && self.height.is_some() {
            width = self.width.unwrap() as f32;
            height = self.height.unwrap() as f32;
        } else if let Some(target_width) = self.width {
            height *= width / target_width as f32;
            width = target_width as f32;
        } else if let Some(target_height) = self.height {
            width *= height / current_height as f32;
            height = target_height as f32;
        }
        (width as usize, height as usize)
    }
}

pub enum Transform {
}
