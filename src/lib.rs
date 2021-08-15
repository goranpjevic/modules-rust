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
        pub struct PublicStruct {
            pub public_field: i32,
            _private_field: i32,
        }
        impl PublicStruct {
            pub fn public_struct_method(value_for_public_field: i32) -> PublicStruct {
                PublicStruct {
                    public_field: value_for_public_field,
                    _private_field: value_for_public_field + 1,
                }
            }
        }
    }
}

// use the 'use' keyword to bring an item into scope:
//use crate::first_module::nested_module;
// we can also use relative paths:
//use self::first_module::nested_module;

// we can use 'pub use' to make the name public outside of this scope.
//pub use crate::first_module::nested_module;
// other external modules referring to this one, can just use nested_module instead of the full
// name.

//use crate::first_module::nested_module::nested_function;
//use crate::first_module::nested_module::PublicStruct;

// we can use nested paths:
use crate::first_module::nested_module::{self, nested_function, PublicStruct};
// with this line we brought into scope the nested_module, the nested_function and PublicStruct.
// 'self' refers to the nested_module in this case.

// use the glob operator (*) to bring all public items into scope.  note: glob can make it harder
// to tell what names are in scope and where a name used in the program was defined.
//use crate::first_module::nested_module::*;

// local name or alias of an item brought into the scope with the 'use' keyword:
use crate::first_module::nested_module::PublicStruct as PubStrct;

// we have to declare all modules that are defined in separate files

mod separate_file;

use separate_file::module_in_a_separate_file;

pub fn public_function() {
    println!("this is a public function.");

    // relative path to the nested_function
    // in order for this to compile, the function as well as the module in which it is located,
    // must be public.
    // first_module does not need to be public, because as it is defined in the same module as this
    // function.
    first_module::nested_module::nested_function();
    // to use an absolute path, we would need to add 'crate::' before 'first_module'

    // the public_struct_method must be public, so that we can use it here.
    let public_struct_instance =
        first_module::nested_module::PublicStruct::public_struct_method(10);
    println!(
        "the value of the public field is: {}",
        public_struct_instance.public_field
    );
    // if we add this line, the code will not compile, because the private_field is not public:
    //println!("the value of the private field is: {}", public_struct_instance.private_field);


    module_in_a_separate_file::function_in_a_separate_file();

    // since we used the 'use' keyword above this function, we can just use nested_module directly
    nested_module::nested_function();
    // we can bring the function into the scope with 'use'.
    // doing this is unidiomatic and it makes it unclear as to where the function was defined.
    nested_function();
    // bringing in structs or enums is idiomatic
    // (we cannot do this if two items or more, brought in by 'use', have the same name)
    let _another_public_struct_instance = PublicStruct::public_struct_method(5);
    let _third_pub_strct_instance = PubStrct::public_struct_method(6);
}
