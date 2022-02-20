fn main() {
    let number_list = vec![45, 12, 76, 89, 33];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);

    let char_list = vec!['z', 'k', 'f', 's', 'a'];
    let largest = get_largest(char_list);
    println!("The largest number is {}", largest);

    print_social_content()
}

fn get_largest<T: PartialOrd + Copy>(vector: Vec<T>) -> T {
    let mut largest = vector[0];

    for number in vector {
        if number > largest {
            largest = number;
        }
    }
    return largest;    
}

// Traits

pub struct NewArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self)-> String;
}

fn print_social_content() {
    let tweet = Tweet {
        username: String::from("@ismael_haytam"),
        content:  String::from("@lorem ipsum"),
        reply: false,
        retweet: false
    };

    let article = NewArticle {
        author: String::from("Ismael Haytam"),
        headline: String::from("The Sky is Falling"),
        content: String::from("The Sky is not actually Falling")
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Tweet summary: {}", article.summarize());
}

