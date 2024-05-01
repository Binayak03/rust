fn main() {
    let string1 = "hello ";
    let string2 = " world";
    
    let concatenate_string = concatenate_string(string1, string2);
    println!("{}", concatenate_string)

}
    fn concatenate_string (str1: &str, str2 : &str) -> String{
       let mut result = String :: from("Hii ");
       result.push_str(str1);
       result.push_str(str2);
       result
    }