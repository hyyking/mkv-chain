#[cfg(all(test, feature = "serde"))]
mod serde_test {
    #[test]
    fn test_serialize_vec() {
        use mkv_chain::linalg::Vec3;
        let tested = Vec3::new([0.1, 0.3, 0.6]); // Inital State
        let ser = serde_json::to_string(&tested).expect("Failed to serialize matrix");
        let _: Vec3 = serde_json::from_str(&ser).unwrap();
    }

    #[test]
    fn test_serialize_mat() {
        use mkv_chain::linalg::Matrix3;
        let t_mat = Matrix3::new([[0.9, 0.0, 0.1], [0.1, 0.3, 0.6], [0.0, 0.1, 0.9]]);
        let ser = serde_json::to_string(&t_mat).expect("Failed to serialize matrix");
        let _: Matrix3 = serde_json::from_str(&ser).unwrap();
    }

    #[test]
    fn test_serialize_chain() {
        use mkv_chain::linalg::Matrix3;
        let t_mat = Matrix3::new([[0.9, 0.0, 0.1], [0.1, 0.3, 0.6], [0.0, 0.1, 0.9]]);
        let ser = serde_json::to_string(&t_mat).expect("Failed to serialize matrix");
        let _: Matrix3 = serde_json::from_str(&ser).unwrap();
    }
}
