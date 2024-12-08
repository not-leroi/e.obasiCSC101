use std::io;
fn main(){
    println!("Welcome to the Public Service APS Level Checker. Please input your name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Hello {},what is your job title?",name);
    println!("Are you an office administrator?");
    println!("An academic?");
    println!("A lawyer?");
    println!("Or a teacher?");
    let mut job_title = String::new();
    io::stdin().read_line(&mut job_title).expect("Failed to read input");
    let job_title = job_title.trim().to_lowercase();
    
    let _job = vec!["office administrator", "academic","lawyer","teacher"];

    if job_title == "office administrator"{
        off_admin();
    }else if job_title == "academic"{
        academic();
    }else if job_title == "lawyer"{
        lawyer();
    }else if job_title == "teacher"{
        teacher();
    }else {
        println!("Your choice is not in our database!");
    }
}

fn off_admin() {
    println!("Here are the different job levels. Please input yours:");
    println!("Intern");
    println!("Administrator");
    println!("Senior Administrator");
    println!("Office Manager");
    println!("Director");
    println!("CEO");
    let _job_level_off_admin = vec!["intern","administrator","senior administrator","office manager","director","ceo"];
    let mut job_level = String::new();
    io::stdin().read_line(&mut job_level).expect("Failed to read input");
    let job_level = job_level.trim().to_lowercase();

    let public_servant = vec!["APS 1-2", "APS 3-5", "APS 5-8","EL1 8-10","EL2 10-13","SES"];
    if job_level == "intern"{
        println!(" You hold the position:{}",public_servant[0] );
    }else if job_level == "administrator"{
        println!("You hold the position: {}",public_servant[1]);
    }else if job_level == "senior administrator"{
        println!("You hold the position: {}", public_servant[2]);
    }else if job_level == "office manager"{
        println!("You hold the position: {}",public_servant[3] );
    }else if job_level == "director"{
        println!("You hold the position: {}",public_servant[4] );
    }else if job_level == "ceo"{
        println!("You hold the position: {}",public_servant[5] );
    }else{
        println!("Choice is not contained in our database");
    }
}

fn academic() {
    println!("Here are the different job levels. Please input yours:");
    println!("Research assistant");
    println!("PhD candidate");
    println!("Post-Doc Researcher");
    println!("Senior Lecturer");
    println!("Dean");
    let _public_servant = vec!["APS 1-2", "APS 3-5", "APS 5-8","EL1 8-10","EL2 10-13","SES"];
    let _job_level_academic = vec!["research assistant","phd candidate","post-doc researcher","senior lecturer","dean"];
    let mut _job_level = String::new();
    io::stdin().read_line(&mut _job_level).expect("Failed to read input");
    let _job_level = _job_level.trim().to_lowercase();
    if _job_level == "research assistant"{
        println!("Your position is: {}",_public_servant[1] );
    }else if _job_level == "phd candidate"{
        println!("Your position is: {}",_public_servant[2] );
    }else if _job_level == "post-doc researcher"{
        println!("Your position is: {}",_public_servant[3] );
    }else if _job_level == "senior lecturer"{
        println!("Your position is: {}",_public_servant[4] );
    }else if _job_level == "dean"{
        println!("Your position is: {}",_public_servant[5]);
    }else {
        println!("Your choice is not in our database");
    }
}

fn lawyer() {
    println!("Here are the different job levels. Please input yours:");
    println!("Paralegal");
    println!("Junior Associate");
    println!("Associate");
    println!("Senior Associate 1-2");
    println!("Senior Associate 3-4");
    println!("Partner");
    let _public_servant = vec!["APS 1-2", "APS 3-5", "APS 5-8","EL1 8-10","EL2 10-13","SES"];
    let _job_level_lawyer = vec!["paralegal","junior associate","associate","senior associate 1-2","senior associate 3-4","partner"];
    let mut _job_level = String::new();
    io::stdin().read_line(&mut _job_level).expect("Failed to read input");
    let _job_level = _job_level.trim().to_lowercase();
    if _job_level == "paralegal"{
        println!("Your position is {}",_public_servant[0] );
    }else if _job_level =="junior associate"{
        println!("Your position is: {}",_public_servant[1] );
    }else if _job_level == "associate"{
        println!("Your position is {}",_public_servant[2] );
    }else if _job_level == "senior associate 1-2"{
        println!("Your position is: {}",_public_servant[3] );
    }else if _job_level == "senior associate 3-4"{
        println!("Your position is: {}",_public_servant[4] );
    }else if _job_level == "partner"{
        println!("Your position is: {}",_public_servant[5] );
    }else {
        println!("Your choice is not in our database.");
    }
}

fn teacher() {
    println!("Here are the different job levels. Please input yours:");
    println!("Placement");
    println!("Classroom teacher");
    println!("Senior Teacher");
    println!("Leading teacher");
    println!("Deputy Principal");
    println!("Principal");
    let _public_servant = vec!["APS 1-2", "APS 3-5", "APS 5-8","EL1 8-10","EL2 10-13","SES"];
    let _job_level_teacher = vec!["placement","classroom teacher","senior teacher","leading teacher","deputy principal","principal"];
    let mut job_level = String::new();
    io::stdin().read_line(&mut job_level).expect("Failed to read input");
    let job_level = job_level.trim().to_lowercase();
    if job_level == "placement"{
        println!("Your position is {}",_public_servant[0] );
    }else if job_level == "classroom teacher"{
        println!("Your position is {}",_public_servant[1] );
    }else if job_level == "senior teacher"{
        println!("Your position is {}",_public_servant[2] );
    }else if job_level == "leading teacher"{
        println!("Your postion is {}",_public_servant[3] );
    }else if job_level == "deputy teacher"{
        println!("Your position is {}",_public_servant[4] );
    }else if job_level == "principal"{
        println!("Your position is {}",_public_servant[5] );
    }else {
        println!("Your choice is not in our database");
    }
}