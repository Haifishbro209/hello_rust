fn main(){
    let name: &str    = "kaloyan"; 
    let mut language: &str = "rust";

    println!("Hello, {}, this is {}", name, language);
    
    language = "JS";
    println!("Hello, {}, this is {}", name, language);

}
