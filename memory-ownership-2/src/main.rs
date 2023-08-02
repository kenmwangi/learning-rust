fn main() {
    let num = 15;
    let single_reference = &num;
    let double_reference = &&single_reference;
    let five_reference = &&&&&num;

    let my_string: String = "Try to make this a String".into();
    println!("{}", my_string);
}
