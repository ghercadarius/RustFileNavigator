use cursive::views::TextView;
use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView, ResizedView};
use cursive::traits::*;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn format_permissions(permission_int: u32) -> String {
    let permissions = format!("{:03o}", permission_int);

    let mut result = String::new();

    for digit in permissions.chars() {
        match digit {
            '4' | '5' | '6' | '7' => result.push('r'),
            _ => result.push('-'),
        }

        match digit {
            '2' | '3' | '6' | '7' => result.push('w'),
            _ => result.push('-'),
        }

        match digit {
            '1' | '3' | '5' | '7' => result.push('x'),
            _ => result.push('-'),
        }
    }

    result
}


pub fn list_dir(path: &str) -> Option<Vec<String>> {
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

fn afis(s: &mut Cursive, path: &String){
    //println!("{}", path);
    if path != ""{
        let output = Command::new("pwd").output().expect("err pwd");
        let mut output = String::from_utf8(output.stdout).expect("error parse");
        output.pop();
        //Command::new("cd").arg(output).output().expect("error cd");
        env::set_current_dir(&path).expect("chdir");
    }
    s.pop_layer();
    let mut fisiere = SelectView::<String>::new()
        .on_submit(|s, item|
            // if item == ".."{
            //     Command::new("cd").args(["."]);
            //     afis(s, &".".to_string())
            // } else {
            //     afis(s, item)
            // });
            afis(s, item));
        //.fixed_size((10, 5));
    let dir_aux = ".";
    if let Some(vec) = list_dir(&dir_aux){
        for ffile in vec{
            let fil2 = ffile.clone();
            fisiere.add_item(ffile, fil2);
        }
    }
    fisiere.add_item("Back", "..".to_string());
    s.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(fisiere))
        .title("FOLDERE"));
}

pub fn run() {

    // LS

    let aux : Vec<String> = env::args().collect();
    let aFolder : String;
    if aux.len() > 1 {
        aFolder = aux[1].clone();
    } else {
        aFolder = ".".to_string();
    }
    // if let Some(vec) = list_dir(&aFolder){
    //     for ffile in vec{
    //         println!("{ffile} permisiuni: ");
    //         let fis = File::open(ffile);
    //         if let Ok(fisier) = fis{
    //             if let Ok(metadata) = fisier.metadata(){
    //                 ///println!("{:?}", metadata.file_type());
    //                 let permisiuni = metadata.permissions();
    //                 let saux = format_permissions(permisiuni.mode());
    //                 println!("{saux}");
    //             }
    //         }
    //     }
    // }


    //LS END


    // TODO 1 - tutorial
    // follow all 3 turorials https://github.com/gyscos/cursive/blob/main/doc/tutorial_1.md

    // let select = SelectView::<String>::new().on_submit(on_submit).fixed_size((10, 5));
    // fn on_submit(s: &mut Cursive, name: &str){
    //     s.pop_layer();
    //     s.add_layer(Dialog::text(format!("Name: {}\nBun", name))
    //         .title("nume nou")
    //         .button("gata", Cursive::quit));
    // }
    
    
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s|s.quit());
    siv.add_layer(Dialog::text(""));
    afis(&mut siv, &aFolder);
    


    // TODO 2 - window
    // create a windows that lists the current directory with all the details about the files / directories

    // TODO 3 - select the directory
    // move with arrow keys through the list that is shown from the current directory
    // if you press enter on a directory, change the output of ls to come from that directory
    siv.run();
}

