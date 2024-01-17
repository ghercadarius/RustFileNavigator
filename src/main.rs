// recreate ls
mod ls;

// create a text ui ls
mod ui;


fn main() {
    // EX 1 - create a ls
    //ls::run();

    // EX 2 - create a text UI ls
    // let aux = ui::list_dir(".");
    // if let Some(vec) = aux{
    //     println!("{:?}", vec);
    // }
    // println!("@");
     ui::run();
}
