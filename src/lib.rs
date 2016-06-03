struct HeaderCard<T> {
    keyword: String,
    value: T,
    comment: Option<String>,
}

impl<T> HeaderCard<T> {
    pub fn without_comment(keyword: &str, value: T) -> HeaderCard<T> {
        HeaderCard {
            keyword: keyword.to_string(),
            value: value,
            comment: None,
        }
    }

    pub fn with_comment(keyword: &str, value: T, comment: &str) -> HeaderCard<T> {
        HeaderCard {
            keyword: keyword.to_string(),
            value: value,
            comment: Some(comment.to_string()),
        }
    }

    pub fn to_string(&self) -> String {
        match self.comment {
            Some(ref comment) => {
                format!("{keyword:8}=", keyword = self.keyword)
            },
            None => {
                "".to_string()
            },
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn building_a_header_card() {
        use super::HeaderCard;

        let card = HeaderCard::without_comment("SIMPLE", true);
        assert_eq!(card.keyword, "SIMPLE");
        assert!(card.value);
        assert_eq!(card.comment, None);

        let card = HeaderCard::without_comment("BITPIX", 32);
        assert_eq!(card.keyword, "BITPIX");
        assert_eq!(card.value, 32);
        assert_eq!(card.comment, None);

        let card = HeaderCard::with_comment("NAXIS", 2, "Number of axes");
        assert_eq!(card.keyword, "NAXIS");
        assert_eq!(card.value, 2);
        assert_eq!(card.comment, Some("Number of axes".to_string()));
    }

    #[test]
    fn rendering_to_string() {
        use super::HeaderCard;

        let card = HeaderCard::with_comment("NAXIS", 2, "Number of axes");
        assert_eq!(card.to_string(),
            //0         1         2         3         4         5         6         7         8
            //01234567890123456789012345678901234567890123456789012345678901234567890123456789
             "NAXIS   =                    2 / Number of axes                                 ");

    }
}
