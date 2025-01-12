use std::fs::File;
use std::io::{self, Write};

// Structure to represent a Commissioner
struct Commissioner {
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

fn main() -> io::Result<()> {
    // Data from separate datasets
    let names = vec![
        "Aiqboqun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieve",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Combine the datasets into a single vector of Commissioner structures
    let mut commissioners = Vec::new();
    for i in 0..names.len() {
        commissioners.push(Commissioner {
            name: names[i].to_string(),
            ministry: ministries[i].to_string(),
            geopolitical_zone: geopolitical_zones[i].to_string(),
        });
    }

    // Display the combined data in the console
    println!("EFCC Report: Merged Datasets");
    println!("-------------------------------------------------------------");
    println!(
        "{:<3} {:<30} {:<20} {:<15}",
        "S/N", "Name", "Ministry", "Geo-Political Zone"
    );
    for (i, commissioner) in commissioners.iter().enumerate() {
        println!(
            "{:<3} {:<30} {:<20} {:<15}",
            i + 1,
            commissioner.name,
            commissioner.ministry,
            commissioner.geopolitical_zone
        );
    }

    // Save the combined data into a file
    let mut file = File::create("efcc_report.txt")?;
    writeln!(file, "EFCC Report: Merged Datasets")?;
    writeln!(file, "-------------------------------------------------------------")?;
    writeln!(
        file,
        "{:<3} {:<30} {:<20} {:<15}",
        "S/N", "Name", "Ministry", "Geo-Political Zone"
    )?;
    for (i, commissioner) in commissioners.iter().enumerate() {
        writeln!(
            file,
            "{:<3} {:<30} {:<20} {:<15}",
            i + 1,
            commissioner.name,
            commissioner.ministry,
            commissioner.geopolitical_zone
        )?;
    }

    println!("\nThe EFCC report has been saved to 'efcc_report.txt'");
    Ok(())
}
