use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Define the categories and their drinks
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    // Open a file to write the data
    let mut file = File::create("drinks_categories.txt")?;

    // Write the categories and drinks into the file
    writeln!(file, "Lager:")?;
    for drink in lager {
        writeln!(file, "- {}", drink)?;
    }

    writeln!(file, "\nStout:")?;
    for drink in stout {
        writeln!(file, "- {}", drink)?;
    }

    writeln!(file, "\nNon-Alcoholic:")?;
    for drink in non_alcoholic {
        writeln!(file, "- {}", drink)?;
    }

    println!("Drinks categories saved to 'drinks_categories.txt'");
    Ok(())
}
