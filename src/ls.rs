use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;


fn list_dir(path: &str) -> Option<Vec<String>> {
    use std::process::Command;
    let deAfisat = 
    match Command::new("ls")
        .args([path])
        .output(){
            Ok(x) => x,
            Err(_) => { return None;}
        };
    if deAfisat.stdout.len() == 0{
        return None;
    }
    let auxString = String::from_utf8(deAfisat.stdout).expect("string invalid");
    let vectString : Vec<String> = auxString.lines().map(|x| x.to_string()).collect();
    Some(vectString)
}

pub fn run() {
    // TODO 1 - arguments
    // use std::env to print the program's first argument
    // https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let aux : Vec<String> = env::args().collect();
    let aFolder : String;
    if aux.len() > 1 {
        aFolder = aux[1].clone();
    } else {
        aFolder = ".".to_string();
    }
    println!("{aFolder}");
    // TODO 2 - select the directory to be listed
    //  - if the first argument exists, that is the directory
    //  - if it does not, the current directory will be listed
    if let Some(vec) = list_dir(&aFolder){
        for ffile in vec{
            println!("{ffile} permisiuni: ");
            let fis = File::open(ffile);
            if let Ok(fisier) = fis{
                if let Ok(metadata) = fisier.metadata(){
                    ///println!("{:?}", metadata.file_type());
                    let permisiuni = metadata.permissions();
                    println!("{}", permisiuni.mode());
                }
            }
        }
    }


    // TODO 3 - print the contents of the directory
    // https://doc.rust-lang.org/std/fs/fn.read_dir.html

    // TODO 4 - print the type of each item (dir, file, link)

    // TODO 5 - print the properties of each directory / file
    // https://doc.rust-lang.org/std/fs/struct.Permissions.html#impl-PermissionsExt-for-Permissions
    // use mode

}
