
struct NewsArticle{
    headline:String,
    author:String,
    content:String
}

struct Tweet{
    username:String,
    content:String
}

trait  Summary {
    fn summarized(&self) -> String;
}


impl Summary for  Tweet {
    fn summarized(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for NewsArticle{
    fn summarized(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}
pub fn rust_trait() {
	
    let first_tweet= Tweet{
        username: String::from("dev_James"),
        content: String::from("i will be a crack dev")
    };
     let article = NewsArticle {
        headline: String::from("Rust takes over the world"),
        author: String::from("Jane Doe"),
        content: String::from("...details..."),
    };

    println!("{}", first_tweet.summarized());
    println!("{}", article.summarized() );
    print_value(String::from("James Victor"));
}

use std::fmt::Display;

fn print_value<T:Display>(value:T){
    println!("{}", value)
}