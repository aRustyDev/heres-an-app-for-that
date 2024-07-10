extern crate clang;

use std::ffi::CString;
use clang::{Arguments, Diagnostic, ErrorCode, TranslationUnit}; 
use std::io::Cursor;
use image::io::Reader as ImageReader; // https://docs.rs/image/latest/image/

enum Language {
    C,
    Cpp,
    Objective_c,
}

struct Data {
    data: String,
    framework: String,
    lang: Language,
}

fn NewData(data_path: &String, framework: &String, lang: &String) -> Result<(Data), Error> {
    Ok(Data { 
        path = data_path, 
        framework = framework, 
        lang = match lang {
            "C" => Language::C,
            "Cpp" => Language::Cpp,
            "Objective_c" => Language::Objective_c,
            Default => return err!("Invalid Language"),
        }
    })
}

fn compile_src(data: Data, output: String) -> Result<(), Error> {
    // Runs twice:
    // 1. ONLY Parse the source code
    // 2. IF no errors -> Compile the source code

    // Specify the source code as a string
    let source_code = "" // Read source from File

    // Convert the source code to a CString for compatibility
    let source_code_str = CString::new(source_code).unwrap();

    // Create arguments for the compiler
    let mut args = Arguments::new();
    // args.add("-fsyntax-only");  // Parse only, don't compile or link
    args.add("-fmodules");  // Enable the 'modules' language feature
    args.add("-framework " + data.framework);  // Add a framework search path for the given framework
    args.add("-o " + output);  // Output file (Ghidra.app/Contents/MacOS/Ghidra)
    args.add("-x " + match data.lang {
        Language::C => "c",
        Language::Cpp => "c++",
        Language::Objective_c => "objective-c",
        Default => Err("Language not supported"),
    })

    // Create the translation unit (Does the actual running of CLang)
    let tu = match TranslationUnit::new(&source_code_str, &args) {
        Ok(tu) => tu,
        Err(err) => {
            println!("Error creating translation unit: {}", err);
            return;
        }
    };

    // Check for diagnostics (errors or warnings)
    // let mut diagnostics = Vec::new();
    // tu.diagnostics(&mut diagnostics);
    // for diag in diagnostics.iter() {
    //     if diag.severity() == ErrorCode::Error {
    //         println!("Error: {}", diag.message());
    //         return;
    //     } else {
    //         println!("Warning: {}", diag.message());
    //     }
    // }

    // You can now use the TranslationUnit object to further analyze the code
    // (e.g., get cursor information, extract declarations, etc.)

    // ... (Your code to process the translation unit)
    Ok(())
}
