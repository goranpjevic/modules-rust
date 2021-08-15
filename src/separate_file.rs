pub mod module_in_a_separate_file {
    pub fn function_in_a_separate_file() {
        println!("this function is in a file called separate_file.rs");
        super::module_in_a_subdirectory::function_in_a_subdirectory();
    }
}

// nested modules in separate files must also be in separate directories
mod module_in_a_subdirectory;
