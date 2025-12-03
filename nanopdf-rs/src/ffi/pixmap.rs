//! C FFI for pixmap - MuPDF compatible
//! Safe Rust implementation using handle-based resource management

use super::{Handle, PIXMAPS};
use super::geometry::fz_irect;
use super::colorspace::{ColorspaceHandle, FZ_COLORSPACE_RGB};

/// Internal pixmap state
pub struct Pixmap {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    n: i32,        // Number of components
    alpha: bool,
    stride: i32,
    samples: Vec<u8>,
    colorspace: ColorspaceHandle,
}

impl Pixmap {
    pub fn new(cs: ColorspaceHandle, width: i32, height: i32, alpha: bool) -> Self {
        let n = super::colorspace::fz_colorspace_n(0, cs) + i32::from(alpha);
        let stride = width * n;
        let size = (stride * height) as usize;

        Self {
            x: 0,
            y: 0,
            width,
            height,
            n,
            alpha,
            stride,
            samples: vec![0u8; size],
            colorspace: cs,
        }
    }

    pub fn with_bbox(cs: ColorspaceHandle, bbox: fz_irect, alpha: bool) -> Self {
        let width = bbox.x1 - bbox.x0;
        let height = bbox.y1 - bbox.y0;
        let n = super::colorspace::fz_colorspace_n(0, cs) + i32::from(alpha);
        let stride = width * n;
        let size = (stride * height).max(0) as usize;

        Self {
            x: bbox.x0,
            y: bbox.y0,
            width,
            height,
            n,
            alpha,
            stride,
            samples: vec![0u8; size],
            colorspace: cs,
        }
    }

    pub fn clear(&mut self) {
        self.samples.fill(0);
    }

    pub fn clear_with_value(&mut self, value: u8) {
        self.samples.fill(value);
    }

    pub fn get_sample(&self, x: i32, y: i32, component: i32) -> Option<u8> {
        if x < self.x || x >= self.x + self.width ||
           y < self.y || y >= self.y + self.height ||
           component < 0 || component >= self.n {
            return None;
        }
        let local_x = x - self.x;
        let local_y = y - self.y;
        let offset = (local_y * self.stride + local_x * self.n + component) as usize;
        self.samples.get(offset).copied()
    }

    pub fn set_sample(&mut self, x: i32, y: i32, component: i32, value: u8) {
        if x < self.x || x >= self.x + self.width ||
           y < self.y || y >= self.y + self.height ||
           component < 0 || component >= self.n {
            return;
        }
        let local_x = x - self.x;
        let local_y = y - self.y;
        let offset = (local_y * self.stride + local_x * self.n + component) as usize;
        if let Some(sample) = self.samples.get_mut(offset) {
            *sample = value;
        }
    }
}

/// Create a new pixmap
#[unsafe(no_mangle)]
pub extern "C" fn fz_new_pixmap(
    _ctx: Handle,
    cs: ColorspaceHandle,
    w: i32,
    h: i32,
    _seps: Handle, // Separations not implemented yet
    alpha: i32,
) -> Handle {
    let cs = if cs == 0 { FZ_COLORSPACE_RGB } else { cs };
    PIXMAPS.insert(Pixmap::new(cs, w, h, alpha != 0))
}

/// Create a new pixmap with bounding box
#[unsafe(no_mangle)]
pub extern "C" fn fz_new_pixmap_with_bbox(
    _ctx: Handle,
    cs: ColorspaceHandle,
    bbox: fz_irect,
    _seps: Handle,
    alpha: i32,
) -> Handle {
    let cs = if cs == 0 { FZ_COLORSPACE_RGB } else { cs };
    PIXMAPS.insert(Pixmap::with_bbox(cs, bbox, alpha != 0))
}

/// Keep (increment ref) pixmap
#[unsafe(no_mangle)]
pub extern "C" fn fz_keep_pixmap(_ctx: Handle, pix: Handle) -> Handle {
    PIXMAPS.keep(pix)
}

/// Drop pixmap reference
#[unsafe(no_mangle)]
pub extern "C" fn fz_drop_pixmap(_ctx: Handle, pix: Handle) {
    let _ = PIXMAPS.remove(pix);
}

/// Get pixmap X origin
#[unsafe(no_mangle)]
pub extern "C" fn fz_pixmap_x(_ctx: Handle, pix: Handle) -> i32 {
    if let Some(p) = PIXMAPS.get(pix) {
        if let Ok(guard) = p.lock() {
            return guard.x;
        }
    }
    0
}

/// Get pixmap Y origin
#[unsafe(no_mangle)]
pub extern "C" fn fz_pixmap_y(_ctx: Handle, pix: Handle) -> i32 {
    if let Some(p) = PIXMAPS.get(pix) {
        if let Ok(guard) = p.lock() {
            return guard.y;
        }
    }
    0
}

/// Get pixmap width
#[unsafe(no_mangle)]
pub extern "C" fn fz_pixmap_width(_ctx: Handle, pix: Handle) -> i32 {
    if let Some(p) = PIXMAPS.get(pix) {
        if let Ok(guard) = p.lock() {
            return guard.width;
        }
    }
    0
}

/// Get pixmap height
#[unsafe(no_mangle)]
pub extern "C" fn fz_pixmap_height(_ctx: Handle, pix: Handle) -> i32 {
    if let Some(p) = PIXMAPS.get(pix) {
        if let Ok(guard) = p.lock() {
            return guard.height;
        }
    }
    0
}

/// Get number of components (including alpha)
#[unsafe(no_mangle)]
pub extern "C" fn fz_pixmap_components(_ctx: Handle, pix: Handle) -> i32 {
    if let Some(p) = PIXMAPS.get(pix) {
        if let Ok(guard) = p.lock() {
            return guard.n;
        }
    }
    0
}

/// Get number of colorants (excluding alpha)
#[unsafe(no_mangle)]
pub extern "C" fn fz_pixmap_colorants(_ctx: Handle, pix: Handle) -> i32 {
    if let Some(p) = PIXMAPS.get(pix) {
        if let Ok(guard) = p.lock() {
            return guard.n - i32::from(guard.alpha);
        }
    }
    0
}

/// Check if pixmap has alpha
#[unsafe(no_mangle)]
pub extern "C" fn fz_pixmap_alpha(_ctx: Handle, pix: Handle) -> i32 {
    if let Some(p) = PIXMAPS.get(pix) {
        if let Ok(guard) = p.lock() {
            return i32::from(guard.alpha);
        }
    }
    0
}

/// Get pixmap stride (bytes per row)
#[unsafe(no_mangle)]
pub extern "C" fn fz_pixmap_stride(_ctx: Handle, pix: Handle) -> i32 {
    if let Some(p) = PIXMAPS.get(pix) {
        if let Ok(guard) = p.lock() {
            return guard.stride;
        }
    }
    0
}

/// Get pixmap bounding box
#[unsafe(no_mangle)]
pub extern "C" fn fz_pixmap_bbox(_ctx: Handle, pix: Handle) -> fz_irect {
    if let Some(p) = PIXMAPS.get(pix) {
        if let Ok(guard) = p.lock() {
            return fz_irect {
                x0: guard.x,
                y0: guard.y,
                x1: guard.x + guard.width,
                y1: guard.y + guard.height,
            };
        }
    }
    fz_irect { x0: 0, y0: 0, x1: 0, y1: 0 }
}

/// Get pixmap colorspace
#[unsafe(no_mangle)]
pub extern "C" fn fz_pixmap_colorspace(_ctx: Handle, pix: Handle) -> ColorspaceHandle {
    if let Some(p) = PIXMAPS.get(pix) {
        if let Ok(guard) = p.lock() {
            return guard.colorspace;
        }
    }
    0
}

/// Clear pixmap to transparent black
#[unsafe(no_mangle)]
pub extern "C" fn fz_clear_pixmap(_ctx: Handle, pix: Handle) {
    if let Some(pixmap) = PIXMAPS.get(pix) {
        if let Ok(mut p) = pixmap.lock() {
            p.clear();
        }
    }
}

/// Clear pixmap to specific value
#[unsafe(no_mangle)]
pub extern "C" fn fz_clear_pixmap_with_value(_ctx: Handle, pix: Handle, value: i32) {
    if let Some(pixmap) = PIXMAPS.get(pix) {
        if let Ok(mut p) = pixmap.lock() {
            p.clear_with_value(value as u8);
        }
    }
}

/// Invert pixmap colors
#[unsafe(no_mangle)]
pub extern "C" fn fz_invert_pixmap(_ctx: Handle, pix: Handle) {
    if let Some(pixmap) = PIXMAPS.get(pix) {
        if let Ok(mut p) = pixmap.lock() {
            let colorants = (p.n - i32::from(p.alpha)) as usize;
            for y in 0..p.height {
                for x in 0..p.width {
                    let offset = (y * p.stride + x * p.n) as usize;
                    for c in 0..colorants {
                        if let Some(sample) = p.samples.get_mut(offset + c) {
                            *sample = 255 - *sample;
                        }
                    }
                }
            }
        }
    }
}

/// Apply gamma correction to pixmap
#[unsafe(no_mangle)]
pub extern "C" fn fz_gamma_pixmap(_ctx: Handle, pix: Handle, gamma: f32) {
    if gamma <= 0.0 {
        return;
    }

    // Pre-compute gamma lookup table
    let mut gamma_table = [0u8; 256];
    for (i, entry) in gamma_table.iter_mut().enumerate() {
        let normalized = (i as f32) / 255.0;
        let corrected = normalized.powf(1.0 / gamma);
        *entry = (corrected * 255.0).round().clamp(0.0, 255.0) as u8;
    }

    if let Some(pixmap) = PIXMAPS.get(pix) {
        if let Ok(mut p) = pixmap.lock() {
            let colorants = (p.n - i32::from(p.alpha)) as usize;
            for y in 0..p.height {
                for x in 0..p.width {
                    let offset = (y * p.stride + x * p.n) as usize;
                    for c in 0..colorants {
                        if let Some(sample) = p.samples.get_mut(offset + c) {
                            *sample = gamma_table[*sample as usize];
                        }
                    }
                }
            }
        }
    }
}

/// Get sample at specific position
#[unsafe(no_mangle)]
pub extern "C" fn fz_get_pixmap_sample(_ctx: Handle, pix: Handle, x: i32, y: i32, n: i32) -> u8 {
    if let Some(p) = PIXMAPS.get(pix) {
        if let Ok(guard) = p.lock() {
            if let Some(sample) = guard.get_sample(x, y, n) {
                return sample;
            }
        }
    }
    0
}

/// Set sample at specific position
#[unsafe(no_mangle)]
pub extern "C" fn fz_set_pixmap_sample(_ctx: Handle, pix: Handle, x: i32, y: i32, n: i32, v: u8) {
    if let Some(pixmap) = PIXMAPS.get(pix) {
        if let Ok(mut p) = pixmap.lock() {
            p.set_sample(x, y, n, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixmap_create() {
        let handle = fz_new_pixmap(0, FZ_COLORSPACE_RGB, 100, 100, 0, 1);
        assert_ne!(handle, 0);

        assert_eq!(fz_pixmap_width(0, handle), 100);
        assert_eq!(fz_pixmap_height(0, handle), 100);
        assert_eq!(fz_pixmap_components(0, handle), 4); // RGB + alpha
        assert_eq!(fz_pixmap_alpha(0, handle), 1);

        fz_drop_pixmap(0, handle);
    }

    #[test]
    fn test_pixmap_clear() {
        let handle = fz_new_pixmap(0, FZ_COLORSPACE_RGB, 10, 10, 0, 0);

        fz_clear_pixmap_with_value(0, handle, 128);
        assert_eq!(fz_get_pixmap_sample(0, handle, 0, 0, 0), 128);

        fz_clear_pixmap(0, handle);
        assert_eq!(fz_get_pixmap_sample(0, handle, 0, 0, 0), 0);

        fz_drop_pixmap(0, handle);
    }

    #[test]
    fn test_pixmap_set_get_sample() {
        let handle = fz_new_pixmap(0, FZ_COLORSPACE_RGB, 10, 10, 0, 0);

        fz_set_pixmap_sample(0, handle, 5, 5, 0, 255);
        assert_eq!(fz_get_pixmap_sample(0, handle, 5, 5, 0), 255);
        assert_eq!(fz_get_pixmap_sample(0, handle, 5, 5, 1), 0);

        fz_drop_pixmap(0, handle);
    }
}
