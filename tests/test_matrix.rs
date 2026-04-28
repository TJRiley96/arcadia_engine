mod test_matrix2 {
    use arcadia_engine::util::math::matrix::Matrix2;

    #[test]
    fn test_matrix2_multiplication() {
        let matrix = Matrix2 {
            data: [
                [1.0, 2.0],
                [3.0, 4.0],
            ],
        };

        let matrix2 = Matrix2 {
            data: [
                [5.0, 6.0],
                [7.0, 8.0],
            ],
        };

        let result = matrix * matrix2;

        assert_eq!(result.data[0][0], 19.0);
        assert_eq!(result.data[0][1], 22.0);
        assert_eq!(result.data[1][0], 43.0);
        assert_eq!(result.data[1][1], 50.0);
    }
}

mod test_matrix4 {
    use arcadia_engine::util::math::matrix::Matrix4;

    #[test]
    fn test_matrix4_identity() {
        let identity = Matrix4::identity();
        assert_eq!(identity.data[0][0], 1.0);
        assert_eq!(identity.data[1][1], 1.0);
        assert_eq!(identity.data[2][2], 1.0);
        assert_eq!(identity.data[3][3], 1.0);
    }

    #[test]
    fn test_matrix4_mul_matrix4() {
        let matrix = Matrix4 {
            data: [
                [0.0, 1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0, 7.0],
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
            ],
        };

        let matrix2 = Matrix4 {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [1.0, 2.0, 3.0, 4.0],
                [1.0, 2.0, 3.0, 4.0],
                [1.0, 2.0, 3.0, 4.0],
            ],
        };

        let result = matrix * matrix2;

        assert_eq!(result.data[0][0], 6.0);
        assert_eq!(result.data[1][1], 44.0);
        assert_eq!(result.data[2][2], 30.0);
        assert_eq!(result.data[3][3], 104.0);
    }
}