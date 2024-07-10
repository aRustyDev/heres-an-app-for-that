use std::fs::{File, remove_file, remove_dir_all};
use std::io::{Write, Error};
use clap::Parser;
use jars::{jar, JarOptionBuilder};
use walkdir::{DirEntry, WalkDir};
use tempfile::Builder;
use crate::clang::{compile_src, Data, Language};
use subprocess::Exec;

fn remove_dir_contents(dir_path: &str) -> Result<(), Error> {
    for entry in WalkDir::new(dir_path)? {
        let entry = entry?;
        if entry.file_type().is_file() {
            remove_file(entry.path())?;
        }
    }
    Ok(())
}

/// Create a MacOS app from source code + Icons
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Language to compile the source code in
    /// Valid values: c, cpp, objective-c
    /// Default: objective-c
    #[arg(short, long, default_value_t = "objective-c")]
    lang: String,

    /// Output file (relative to the pwd)
    /// Default: Ghidra.app/Contents/MacOS/MacNAppd
    #[arg(short, long, default_value_t = "Ghidra.app/Contents/MacOS/MacNAppd")]
    output: String,

    /// Path to data files (relative to the pwd)
    /// Default: ../data
    #[arg(short, long, default_value_t = "../data")]
    data: String,

    /// framework search path
    /// Default: Foundation
    #[arg(short, long, default_value_t = "Foundation")]
    framework: String,
}

fn main() {
    let args = Args::parse();

    // Create a temporary directory tree
    let tmp_dir = Builder::new().prefix("MacNApp").tempdir()?;
    // // Make Files
    // for f in [""]? {
    //     let file_path = tmp_dir.path().join(f);
    //     let mut tmp_file = File::create(file_path)?;
    // }
    // Make Directories
    for d in ["APP.iconset", "APP.app/Contents/MacOS/", "APP.app/Contents/Resources/", "docking/widgets/filechooser/"]? {
        let file_path = tmp_dir.path().join(d);
        let mut tmp_file = File::create(file_path)?;
    }
    let file_path = tmp_dir.path().join("APP.iconset");
    let mut tmp_file = File::create(file_path)?;
    
    // Run Clang
    if let data = NewData(&args.data, &args.framework, &args.lang).unwrap() { // TODO: Error Handling
        compile_src(data, args.output).unwrap();
    };

    // Clean out Ghidra.app/Contents/Resources/ghidra
    remove_dir_contents("Ghidra.app/Contents/Resources/ghidra")?;

    // Copy files from OUTPUT to Ghidra.app/Contents/Resources/ghidra
    for entry in WalkDir::new(output)? { // TODO: review this
        let entry = entry?;
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap();
            let dest_path = tmp_dir.path().join("APP.app/Contents/Resources/ghidra/").join(file_name);
            std::fs::copy(file_path, dest_path)?;
        }
    }

    // TODO: Implement (sed "s/bg Ghidra/fg Ghidra/" < "$1/ghidraRun" > Ghidra.app/Contents/Resources/ghidra/ghidraRun)

    // TODO: Implement (Get source launch.properties from $OUTPUT/support/launch.properties, update, & apply to Ghidra.app/Contents/Resources/ghidra/support/launch.properties)

    // Set Ghidra.app/Contents/PkgInfo contents
    let mut file = File::create("foo.txt")?;
    file.write_all(b"APPL????")?;

    // Extract JAR file 
    let jar = jars::jar("Ghidra.app/Contents/Resources/ghidra/Ghidra/Framework/Gui/lib/Gui.jar", JarOptionBuilder::builder().target("images/GhidraIcon256.png").build();)?;

    // Convert PNG
    //      if [ "$( (sw_vers -productVersion; echo "11.0") | sort -V | head -n 1)" = "11.0" ]; then
    //      	convert \( -size 1024x1024 canvas:none -fill white -draw 'roundRectangle 100,100 924,924 180,180' \) \( +clone -background black -shadow 25x12+0+12 \) +swap -background none -layers flatten -crop 1024x1024+0+0 \( images/GhidraIcon256.png -resize 704x704 -gravity center \) -composite GhidraIcon.png
    //      else
    //      	mv images/GhidraIcon256.png GhidraIcon.png
    //      fi
    // create_iconset()
    
    // Update JAR archive
    //  - Ghidra.app/Contents/Resources/ghidra/Ghidra/Framework/Generic/lib/Generic.jar
    //  - images/GhidraIcon${size}.png
    let dir_checksum = {
        Exec::shell("jar -u -f Ghidra.app/Contents/Resources/ghidra/Ghidra/Framework/Generic/lib/Generic.jar images/GhidraIcon${size}.png")
    }.capture()?.stdout_str();
    

    // TODO: Implement (Convert IconSet to ICNS)
    // Copy New ICNS to Ghidra.app/Contents/Resources/Ghidra.icns
    // TODO: Implement (SetFile -a B Ghidra.app)
    // CP "Info.plist" Ghidra.app/Contents/Info.plist
    // CP "GhidraFileChooser.java" docking/widgets/filechooser/GhidraFileChooser.java

    // Javac -cp "$(find Ghidra.app -regex '.*\.jar' | tr '\n' ':')" docking/widgets/filechooser/GhidraFileChooser.java
    let dir_checksum = {
        Exec::shell("Javac -cp "$(find Ghidra.app -regex '.*\.jar' | tr '\n' ':')" docking/widgets/filechooser/GhidraFileChooser.java")
    }.capture()?.stdout_str();
    
    // cp -R docking Ghidra.app/Contents/Resources/Ghidra/ghidra/patch/
    // CP "OpenGhidra.java" OpenGhidra.java
    
    // javac OpenGhidra.java
    let dir_checksum = {
        Exec::shell("javac OpenGhidra.java")
    }.capture()?.stdout_str();
    
    // CP OpenGhidra.class Ghidra.app/Contents/Resources
    // CP "OpenGhidraAgent.java" OpenGhidraAgent.java
    
    // javac -cp "$(find Ghidra.app -regex '.*\.jar' | tr '\n' ':')" OpenGhidraAgent.java
    let dir_checksum = {
        Exec::shell("javac -cp "$(find Ghidra.app -regex '.*\.jar' | tr '\n' ':')" OpenGhidraAgent.java")
    }.capture()?.stdout_str();

    // CP "manifest" > manifest

    // jar --create --file OpenGhidra.jar --manifest manifest OpenGhidraAgent*.class
    let dir_checksum = {
        Exec::shell("jar --create --file OpenGhidra.jar --manifest manifest OpenGhidraAgent*.class")
    }.capture()?.stdout_str();

    // cp OpenGhidra.jar Ghidra.app/Contents/Resources

    
}

// data
// ├── Ghidra.app/
// │   ├── Ghidra.iconset/
// │   ├── docking/
// │   │   └── widgets/
// │   │       └── filechooser/
// │   │           └── GhidraFileChooser.java
// │   └── Ghidra.app/
// │       └── Contents/
// │           ├── Info.plist
// │           ├── MacOS/
// │           └── Resources/
// │               ├── ghidra/Ghidra/Framework/Gui/lib/Gui.jar
// │               ├── ghidra/Ghidra/Framework/Generic/lib/Generic.jar
// │               └── Ghidra.icns
// └── ghidra
//     ├── GhidraFileChooser.java
//     ├── OpenGhidra.java
//     ├── OpenGhidraAgent.java
//     ├── iconset.json
//     ├── info.plist
//     └── manifest

// ", "APP.app/Contents/MacOS/", "APP.app/Contents/Resources/", "docking/widgets/filechooser/