pub mod vec2 {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Vec2<T: Copy> {
        pub x: T,
        pub y: T
    }

    pub fn vec2<T: Copy>(x: T, y: T) -> Vec2<T> {
        Vec2 {
            x,
            y
        }
    }

    impl From<(f64, f64)> for Vec2<f32> {
        fn from(f: (f64, f64)) -> Self {
            Vec2 {
                x: f.0 as f32,
                y: f.1 as f32
            }
        }
    }
}

pub mod vec3 {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Vec3<T: Copy> {
        pub x: T,
        pub y: T,
        pub z: T
    }

    pub fn vec3<T: Copy>(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {
            x,
            y,
            z
        }
    }
}
