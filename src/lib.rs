// the root module for this library is named 'crate'

mod first_module {
    fn function_in_the_first_module() {
        println!("this is a function in the first module.");
    }

    pub mod nested_module {
        pub fn nested_function() {
            println!("this is a function in the nested module of the first module.");

            // we can refer to the parent module with the 'super' keyword
            // the function_in_the_first_module does not need to be public to be referenced here.
            super::function_in_the_first_module();
        }

        // in public structs, we must assign each field individually, whether it should be public or
        // private. in public enums, all variants are public.
        pub struct Public_struct {
            pub public_field: i32,
            private_field: i32,
        }
        impl Public_struct {
            pub fn public_struct_method(value_for_public_field: i32) -> Public_struct {
                Public_struct {
                    public_field: value_for_public_field,
                    private_field: value_for_public_field + 1,
                }
            }
        }
    }
}

pub fn public_function {
    println!("this is a public function.");

    // relative path to the nested_function
    // in order for this to compile, the function as well as the module in which it is located,
    // must be public.
    // first_module does not need to be public, because as it is defined in the same module as this
    // function.
    first_module::nested_module::nested_function();
    // to use an absolute path, we would need to add 'crate::' before 'first_module'

    // the public_struct_method must be public, so that we can use it here.
    let public_struct_instance = first_module::nested_module::Public_struct::public_struct_method(10);
    println!("the value of the public field is: {}", public_struct_instance.public_field);
    // if we add this line, the code will not compile, because the private_field is not public:
    //println!("the value of the private field is: {}", public_struct_instance.private_field);
}
