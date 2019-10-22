fn main() {
    fn print_int(i: i32) {
        println!("{}", i);
    }

    {
        // ----- Box -----
        let my_one = 1;
        print_int(my_one);

        let my_two = Box::new(2);
        print_int(*my_two);
    }
}
