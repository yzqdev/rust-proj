#[allow(dead_code)]
pub trait Summary {
    fn summarize(&self) -> String;
     fn print_content(&self);
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
      fn print_content(&self){
        println!("{}",self.content);
    }
    
}
pub struct Point<T> {
    x: T,
    y: T,
}

pub fn trait_main(){
    let arcicle= NewsArticle{
        headline:String::from("Rust is awesome"),
        location:String::from("China"),
        author:String::from("Jone"),
        content:String::from("Rust is a great language"),
    };
   let msg= arcicle.summarize();
   println!("{}",msg);
   let p=Point{x:5,y:10};
   print!("p.x={}\np.y={}",p.x,p.y)

    
}