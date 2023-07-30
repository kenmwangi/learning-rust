fn main() {
    {
        let _x = 10; // x is the owner of value 10

        let mut str = String::from("rust");
        str.push_str(" program"); // append literal to a String

        println!("str = {}", str);

        println!(
            "ptr = {:?}\nlength = {} \ncapacity = {}",
            str.as_ptr(),
            str.len(),
            str.capacity()
        )

    }
        let s = String::from('rust');
        foo_string_ref(&s);
        println!("{}", s);
}


fn foo_string_ref(ss: &string){
    println!("{}", ss);
}