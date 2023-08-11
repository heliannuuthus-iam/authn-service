mod utils_test {
    use forum_server::common::utils;

    #[test]
    fn test_desensitize_email_username_le_4() {
        assert_eq!(
            utils::desensitize_email("test@gmail.com"),
            "t****t@gmail.com"
        )
    }

    #[test]
    fn test_desensitize_email_username_gt_4() {
        assert_eq!(
            utils::desensitize_email("test2@gmail.com"),
            "te****t2@gmail.com"
        )
    }

    #[test]
    fn test_desensitize_full_name() {
        assert_eq!(utils::desensitize_text("我的"), "我*");
    }

    #[test]
    fn test_desensitize_full_name_grater_than_2() {
        assert_eq!(utils::desensitize_text("我们的"), "我*的");
    }
}
