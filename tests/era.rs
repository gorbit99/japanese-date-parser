#[cfg(test)]
mod era {
    use japanese_date_parser::parse;

    #[test]
    fn test_era() {
        assert_eq!(
            parse("平成１２年０３月１２日"),
            Some(chrono::NaiveDate::from_ymd_opt(2000, 3, 12).unwrap()),
            "平成１２年０３月１２日"
        );
        assert_eq!(
            parse("平成24年3月12日"),
            Some(chrono::NaiveDate::from_ymd_opt(2012, 3, 12).unwrap()),
            "平成24年3月12日"
        );
        assert_eq!(
            parse("明治24年3月12日"),
            Some(chrono::NaiveDate::from_ymd_opt(1891, 3, 12).unwrap()),
            "明治24年3月12日"
        );
        assert_eq!(
            parse("安永24年3月12日"),
            Some(chrono::NaiveDate::from_ymd_opt(1795, 3, 12).unwrap()),
            "安永24年3月12日"
        );
    }

    #[test]
    fn test_shortened() {
        assert_eq!(
            parse("H24.03.12"),
            Some(chrono::NaiveDate::from_ymd_opt(2012, 3, 12).unwrap()),
            "H24.03.12"
        );
        assert_eq!(
            parse("平24.03.12"),
            Some(chrono::NaiveDate::from_ymd_opt(2012, 3, 12).unwrap()),
            "平24.03.12"
        );
    }
}
