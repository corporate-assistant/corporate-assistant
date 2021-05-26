use structopt::StructOpt;

pub use record::recorder::Recorder; 




/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// Text of utterance to be spoken
    #[structopt(short = "t", long = "text")]
    text: String,
    /// name of database file
    #[structopt(short = "db", long = "database", default_value="all")]
    database: std::path::PathBuf,
}




/// potential usage record --database=<desired database file name to write to> --text=<"Text to be spoken">
fn main() -> Result<(), String> {

    let record_args = Cli::from_args();
    println!("text: {}", record_args.text);
    println!("database: {:?}", record_args.database);

    let myrecorder = Recorder::new();

    // "all" means create three databases : train.txt, test.txt, dev.txt
    if record_args.database.to_str().expect("Improper name of database given") == "all" {
        println!("DataBase: {} . Please say: {}", "train.csv", record_args.text);
        myrecorder.record_and_store(&std::path::PathBuf::from("train.csv"), &record_args.text).unwrap();
        println!("DataBase: {} . Please say: {}", "dev.csv", record_args.text);
        myrecorder.record_and_store(&std::path::PathBuf::from("dev.csv"), &record_args.text).unwrap();
        println!("DataBase: {} . Please say: {}", "test.csv", record_args.text);
        myrecorder.record_and_store(&std::path::PathBuf::from("test.csv"), &record_args.text).unwrap();
    } else {
        println!("DataBase: {:?} . Please say: {}", record_args.database, record_args.text);
        myrecorder.record_and_store(&record_args.database, &record_args.text).unwrap();
    }
    
    Ok(())
}
