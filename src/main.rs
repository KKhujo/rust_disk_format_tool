use std::io::{self, Write};
use std::process::Command;

fn main() {

    let partition = prompt("Enter the partition path (e.g., /dev/sdb1):");

    if !partition_exists(&partition) {
        eprintln!("The partition {} does not exist or is invalid.", partition);
        std::process::exit(1);
    }

    println!("Select the filesystem type:");
    println!("1. ext4");
    println!("2. vfat");
    println!("3. ntfs");
    println!("4. exfat");
    let fs_choice = prompt("Enter your choice (1-4):");

    let filesystem = match fs_choice.trim() {
        "1" => "ext4",
        "2" => "vfat",
        "3" => "ntfs",
        "4" => "exfat",
        _ => {
            eprintln!("Invalid choice. Please enter a number between 1 and 4.");
            std::process::exit(1);
        }
    };

    let confirm = prompt(&format!(
        "Are you sure you want to format {} as {}? This will erase all data on the partition. (yes/no):",
        partition, filesystem
    ));

    if confirm.trim() != "yes" {
        println!("Operation cancelled.");
        std::process::exit(1);
    }


    println!("Formatting {} as {}...", partition, filesystem);
    let output = Command::new("sudo")
        .arg("mkfs")
        .arg("-t")
        .arg(filesystem)
        .arg(&partition)
        .output()
        .expect("Failed to execute mkfs command");

    if output.status.success() {
        println!(
            "The partition {} has been formatted as {} successfully.",
            partition, filesystem
        );
    } else {
        eprintln!("Failed to format the partition. Please check the partition path and try again.");
        io::stderr().write_all(&output.stderr).unwrap();
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn partition_exists(partition: &str) -> bool {
    Command::new("lsblk")
        .arg(partition)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
