fn main() {
    struct Employee {
        name: String,
        age: u8,
        email_id: String,
        experience: u8,
    }

    impl Employee {
        // Employee fields acessed through "self" using dot operator
        fn get_name(&self) -> &str {
            &self.name
        }

        fn set_age(&mut self, age: u8) {
            self.age = age;
        }

        fn print_employee(&self) {
            println!(
                "Name : {}, Age : {}, email_id = {}, experience = {}",
                self.name, self.age, self.email_id, self.experience
            )
        }
    }
}
