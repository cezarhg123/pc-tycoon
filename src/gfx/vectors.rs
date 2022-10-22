pub mod vec2 {
    #[derive(Debug, Clone, Copy)]
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
}

pub mod vec3 {
    #[derive(Debug, Clone, Copy)]
    pub struct Vec3<T: Copy> {
        x: T,
        y: T,
        z: T
    }

    pub fn vec3<T: Copy>(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {
            x,
            y,
            z
        }
    }
}
