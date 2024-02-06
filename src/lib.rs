pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl  Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn news_article_summary_formats_correctly() {
        let article = NewsArticle {
            headline: String::from("Breaking News"),
            location: String::from("World"),
            author: String::from("Author"),
            content: String::from("Content"),
        };

        assert_eq!(article.summarize(), "Breaking News, by Author (World)");
    }

    #[test]
    fn tweet_summary_formats_correctly() {
        let tweet = Tweet {
            username: String::from("User"),
            content: String::from("Content"),
            reply: false,
            retweet: false,
        };

        assert_eq!(tweet.summarize(), "User: Content");
    }

    #[test]
    fn news_article_summary_with_empty_fields() {
        let article = NewsArticle {
            headline: String::from(""),
            location: String::from(""),
            author: String::from(""),
            content: String::from(""),
        };

        assert_eq!(article.summarize(), ", by  ()");
    }

    #[test]
    fn tweet_summary_with_empty_fields() {
        let tweet = Tweet {
            username: String::from(""),
            content: String::from(""),
            reply: false,
            retweet: false,
        };

        assert_eq!(tweet.summarize(), ": ");
    }
}
