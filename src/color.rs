use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(color: &Color) -> String {
    let ir = (color.x() * 255.999) as i32;
    let ig = (color.y() * 255.999) as i32;
    let ib = (color.z() * 255.999) as i32;
    format!("{} {} {}", ir, ig, ib)
}

#[cfg(test)]
mod tests {

    #[test]
    fn write_color() {
        let reference = "255 0 0";
        assert_eq!(
            reference,
            super::write_color(&crate::vec3::Vec3::new(1.0, 0.0, 0.0))
        );
    }

    #[test]
    fn write_color_wrong() {
        let reference = "255 a 0";
        assert_ne!(
            reference,
            super::write_color(&crate::vec3::Vec3::new(1.0, 0.0, 0.0))
        );
    }
}
