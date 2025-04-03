#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsva {
    h: f32,
    s: f32,
    v: f32,
    a: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsla {
    h: f32,
    s: f32,
    l: f32,
    a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn hex(v: u64) -> Self {
        Self::new(
            ((v >> 24) & 0xff) as u8,
            ((v >> 16) & 0xff) as u8,
            ((v >> 8) & 0xff) as u8,
            (v & 0xff) as u8,
        )
    }
}

impl Hsva {
    pub fn new(h: f32, s: f32, v: f32, a: u8) -> Self {
        Self { h, s, v, a }
    }

    pub fn hex(rgba_v: u64) -> Self {
        Self::from(Rgba::hex(rgba_v))
    }
}

impl Hsla {
    pub fn new(h: f32, s: f32, l: f32, a: u8) -> Self {
        Self { h, s, l, a }
    }

    pub fn hex(rgba_v: u64) -> Self {
        Self::from(Rgba::hex(rgba_v))
    }
}

impl From<Hsva> for Rgba {
    fn from(hsva: Hsva) -> Self {
        let c = hsva.v * hsva.s;
        let m = hsva.v - c;
        let (r, g, b) = rgb_from_hcm(hsva.h, c, m);

        Self::new(r, g, b, hsva.a)
    }
}

impl From<Hsla> for Rgba {
    fn from(hsla: Hsla) -> Self {
        let c = (1.0 - (2.0 * hsla.l - 1.0).abs()) * hsla.s;
        let m = hsla.l - (c / 2.0);
        let (r, g, b) = rgb_from_hcm(hsla.h, c, m);

        Self::new(r, g, b, hsla.a)
    }
}

impl From<Rgba> for Hsva {
    fn from(rgba: Rgba) -> Self {
        let r = rgba.r as f32 / 255.0;
        let g = rgba.g as f32 / 255.0;
        let b = rgba.b as f32 / 255.0;

        let (h, v, c) = hue_from_rgb(r, g, b);
        let s = if v == 0.0 { 0.0 } else { c / v };

        Self::new(h, s, v, rgba.a)
    }
}

impl From<Hsla> for Hsva {
    fn from(hsla: Hsla) -> Self {
        let v = hsla.l + hsla.s * hsla.l.min(1.0 - hsla.l);
        let s = if v == 0.0 {
            0.0
        } else {
            2.0 * (1.0 - (hsla.l / v))
        };

        Self::new(hsla.h, s, v, hsla.a)
    }
}

impl From<Rgba> for Hsla {
    fn from(rgba: Rgba) -> Self {
        let r = rgba.r as f32 / 255.0;
        let g = rgba.g as f32 / 255.0;
        let b = rgba.b as f32 / 255.0;

        let (h, v, c) = hue_from_rgb(r, g, b);
        let l = v - c / 2.0;
        let s = if l == 0.0 || l == 1.0 {
            0.0
        } else {
            c / (1.0 - (2.0 * v - c - 1.0).abs())
        };

        Self::new(h, s, l, rgba.a)
    }
}

impl From<Hsva> for Hsla {
    fn from(hsva: Hsva) -> Self {
        let l = hsva.v * (1.0 - (hsva.s / 2.0));
        let s = if l == 0.0 || l == 1.0 {
            0.0
        } else {
            (hsva.v - l) / (l.min(1.0 - l))
        };

        Self::new(hsva.h, s, l, hsva.a)
    }
}

pub trait GlColor {
    fn gl_color(&self) -> (f32, f32, f32, f32);
}

impl GlColor for Rgba {
    fn gl_color(&self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }
}

impl GlColor for Hsva {
    fn gl_color(&self) -> (f32, f32, f32, f32) {
        Rgba::from(*self).gl_color()
    }
}

impl GlColor for Hsla {
    fn gl_color(&self) -> (f32, f32, f32, f32) {
        Rgba::from(*self).gl_color()
    }
}

fn rgb_from_hcm(h: f32, c: f32, m: f32) -> (u8, u8, u8) {
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());

    let (r_1, b_1, g_1) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else if h < 360.0 {
        (c, 0.0, x)
    } else {
        unreachable!()
    };

    (
        ((r_1 + m) * 255.0).round() as u8,
        ((g_1 + m) * 255.0).round() as u8,
        ((b_1 + m) * 255.0).round() as u8,
    )
}

fn hue_from_rgb(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let x_max = r.max(g).max(b);
    let x_min = r.min(g).min(b);
    let c = x_max - x_min;

    (
        60.0 * if c == 0.0 {
            0.0
        } else if x_max == r {
            ((g - b) / c) % 6.0
        } else if x_max == g {
            ((b - r) / c) + 2.0
        } else if x_max == b {
            ((r - g) / c) + 4.0
        } else {
            unreachable!()
        },
        x_max,
        c,
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn rgb_to_hsv() {
        let rgb = Rgba::hex(0x60bfbfff);
        let hsv = Hsva::from(rgb);

        assert_relative_eq!(hsv.h, 180.0, epsilon = 0.01);
        assert_relative_eq!(hsv.s, 0.50, epsilon = 0.01);
        assert_relative_eq!(hsv.v, 0.75, epsilon = 0.01);
        assert_eq!(hsv.a, rgb.a);
    }

    #[test]
    fn rgb_to_hsl() {
        let rgb = Rgba::hex(0x9fdfdfff);
        let hsl = Hsla::from(rgb);

        assert_relative_eq!(hsl.h, 180.0, epsilon = 0.01);
        assert_relative_eq!(hsl.s, 0.50, epsilon = 0.01);
        assert_relative_eq!(hsl.l, 0.75, epsilon = 0.01);
        assert_eq!(hsl.a, rgb.a);
    }

    #[test]
    fn hsv_to_rgb() {
        let hsv = Hsva::new(180.0, 0.50, 0.75, 255);
        let rgb = Rgba::from(hsv);

        assert_eq!(rgb.r, 0x60);
        assert_eq!(rgb.g, 0xbf);
        assert_eq!(rgb.b, 0xbf);
        assert_eq!(rgb.a, hsv.a);
    }

    #[test]
    fn hsv_to_hsl() {
        let hsv = Hsva::new(180.0, 0.50, 0.75, 255);
        let hsl = Hsla::from(hsv);

        assert_relative_eq!(hsl.h, 180.0, epsilon = 0.01);
        assert_relative_eq!(hsl.s, 0.43, epsilon = 0.01);
        assert_relative_eq!(hsl.l, 0.56, epsilon = 0.01);
        assert_eq!(hsl.a, hsv.a);
    }

    #[test]
    fn hsl_to_rgb() {
        let hsl = Hsla::new(180.0, 0.50, 0.75, 255);
        let rgb = Rgba::from(hsl);

        assert_eq!(rgb.r, 0x9f);
        assert_eq!(rgb.g, 0xdf);
        assert_eq!(rgb.b, 0xdf);
        assert_eq!(rgb.a, hsl.a);
    }

    #[test]
    fn hsl_to_hsv() {
        let hsl = Hsla::new(180.0, 0.50, 0.75, 255);
        let hsv = Hsva::from(hsl);

        assert_relative_eq!(hsv.h, 180.0, epsilon = 0.01);
        assert_relative_eq!(hsv.s, 0.28, epsilon = 0.01);
        assert_relative_eq!(hsv.v, 0.87, epsilon = 0.01);
        assert_eq!(hsv.a, hsl.a);
    }
}
