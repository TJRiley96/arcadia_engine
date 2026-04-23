

mod test_vector {
    use rust_opengl::util::math::vector::{Vector2, Vector3, Vector4};

    #[test]
    fn test_vector2_add_vector() {
        let v1 = Vector2 { x: 1.0, y: 2.0 };
        let v2 = Vector2 { x: 3.0, y: 4.0 };
        let result = v1 + v2;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }

    #[test]
    fn test_vector2_sub_vector() {
        let v1 = Vector2 { x: 5.0, y: 7.0 };
        let v2 = Vector2 { x: 2.0, y: 3.0 };
        let result = v1 - v2;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 4.0);
    }

    #[test]
    fn test_vector2_mul_scalar() {
        let v1 = Vector2 { x: 2.0, y: 3.0 };
        let scalar = 4.0;
        let result = v1 * scalar;
        assert_eq!(result.x, 8.0);
        assert_eq!(result.y, 12.0);
    }

    #[test]
    fn test_vector2_div_scalar() {
        let v1 = Vector2 { x: 8.0, y: 12.0 };
        let scalar = 4.0;
        let result = v1 / scalar;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 3.0);
    }

    #[test]
    fn test_vector3_add_vector() {
        let v1 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
        let result = v1 + v2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_vector3_add_scalar() {
        let v1 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
        let scalar: f32 = 4.0;
        let result = v1 + scalar;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, 7.0);
    }

    #[test]
    fn test_vector3_sub_vector() {
        let v1 = Vector3 { x: 5.0, y: 7.0, z: 9.0 };
        let v2 = Vector3 { x: 2.0, y: 3.0, z: 4.0 };
        let result = v1 - v2;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 5.0);
    }

    #[test]
    fn test_vector3_sub_scalar() {
        let v1 = Vector3 { x: 5.0, y: 7.0, z: 9.0 };
        let scalar: f32 = 2.0;
        let result = v1 - scalar;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 5.0);
        assert_eq!(result.z, 7.0);
    }

    #[test]
    fn test_vector3_mul_scalar() {
        let v1 = Vector3 { x: 2.0, y: 3.0, z: 4.0 };
        let scalar = 5.0;
        let result = v1 * scalar;
        assert_eq!(result.x, 10.0);
        assert_eq!(result.y, 15.0);
        assert_eq!(result.z, 20.0);
    }

    #[test]
    fn test_vector3_div_scalar() {
        let v1 = Vector3 { x: 10.0, y: 15.0, z: 20.0 };
        let scalar = 5.0;
        let result = v1 / scalar;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 4.0);
    }

    #[test]
    fn test_vector4_add_vector() {
        let v1 = Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
        let v2 = Vector4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
        let result = v1 + v2;
        assert_eq!(result.x, 6.0);
        assert_eq!(result.y, 8.0);
        assert_eq!(result.z, 10.0);
        assert_eq!(result.w, 12.0);
    }

    #[test]
    fn test_vector4_sub_vector() {
        let v1 = Vector4 { x: 5.0, y: 7.0, z: 9.0, w: 11.0 };
        let v2 = Vector4 { x: 2.0, y: 3.0, z: 4.0, w: 5.0 };
        let result = v1 - v2;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 5.0);
        assert_eq!(result.w, 6.0);
    }

    #[test]
    fn test_vector4_mul_scalar() {
        let v1 = Vector4 { x: 2.0, y: 3.0, z: 4.0, w: 5.0 };
        let scalar = 6.0;
        let result = v1 * scalar;
        assert_eq!(result.x, 12.0);
        assert_eq!(result.y, 18.0);
        assert_eq!(result.z, 24.0);
        assert_eq!(result.w, 30.0);
    }

    #[test]
    fn test_vector4_div_scalar() {
        let v1 = Vector4 { x: 12.0, y: 18.0, z: 24.0, w: 30.0 };
        let scalar = 6.0;
        let result = v1 / scalar;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 4.0);
        assert_eq!(result.w, 5.0);
    }

}
