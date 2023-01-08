#[cfg(test)]
mod gregorian {
    use japanese_date_parser::parse;

    #[test]
    fn test_gregorian() {
        assert_eq!(
            parse("２０１２年０３月１２日"),
            Some(chrono::NaiveDate::from_ymd_opt(2012, 3, 12).unwrap()),
            "２０１２年０３月１２日"
        );
        assert_eq!(
            parse("2012年03月12日"),
            Some(chrono::NaiveDate::from_ymd_opt(2012, 3, 12).unwrap()),
            "2012年03月12日"
        );
    }

    #[test]
    fn test_with_weekday() {
        assert_eq!(
            parse("２０１２年０３月１２日 (月)"),
            Some(chrono::NaiveDate::from_ymd_opt(2012, 3, 12).unwrap()),
            "２０１２年０３月１２日 (月)"
        );
        assert_eq!(
            parse("２０１２年０３月１２日 (月曜日)"),
            Some(chrono::NaiveDate::from_ymd_opt(2012, 3, 12).unwrap()),
            "２０１２年０３月１２日 (月曜日)"
        );
        assert_eq!(
            parse("２０１２年０３月１２日（月）"),
            Some(chrono::NaiveDate::from_ymd_opt(2012, 3, 12).unwrap()),
            "２０１２年０３月１２日（月）"
        );
        assert_eq!(
            parse("２０１２年０３月１２日（月曜日）"),
            Some(chrono::NaiveDate::from_ymd_opt(2012, 3, 12).unwrap()),
            "２０１２年０３月１２日（月曜日）"
        );
    }

    #[test]
    fn test_invalid_dates() {
        assert_eq!(
            parse("２０１２０３月１２日（月曜日）"),
            None,
            "２０１２０３月１２日（月曜日）"
        );
        assert_eq!(
            parse("２０１２年０３月１２日（月曜日"),
            None,
            "２０１２年０３月１２日（月曜日"
        );
        assert_eq!(
            parse("２０１２年０３月１２日（月日）"),
            None,
            "２０１２年０３月１２日（月日）"
        );
    }

    #[test]
    fn test_shortened() {
        assert_eq!(
            parse("12/03/12"),
            Some(chrono::NaiveDate::from_ymd_opt(2012, 3, 12).unwrap()),
            "12/03/12"
        );
        assert_eq!(
            parse("12.03.12"),
            Some(chrono::NaiveDate::from_ymd_opt(2012, 3, 12).unwrap()),
            "12.03.12"
        );
    }
}
