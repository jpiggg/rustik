fn main() {
    let str1 = String::from("hello");
    let test = move | mut str: String | {
        str.push_str(&String::from(" world"));  
        str
    };
    
    let tested = test(str1);

    println!("{:?}", tested);
}`