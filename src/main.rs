mod core;

use crate::core::{err::ExitStderr, session::GrepOem, status::StatusMark, utils::build_banner};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    save: Option<String>
}

fn main() {
    let args = Args::parse();
    let session = GrepOem::new();

    println!("{} Opening ACPI table..", StatusMark::Info);

    match session.open_msdm() {
        Ok(oem_key) => {
            let result = format!("OEM Key: {}", oem_key.value);

            println!("\n{} Found!", StatusMark::Success);

            if let Some(build_banner) = build_banner(&result) {
                println!("\n{}", build_banner);
                println!();
            } else {
                println!("{}", result);
            }

            if let Some(ref file) = args.save {
                session.save_key(oem_key, &file)
                    .map_err(|err| {
                        ExitStderr::quit(format!("{} Error saving key: {}", StatusMark::Error, err), -1)
                    })
                    .ok();
            }
        },
        Err(err) => {
            ExitStderr::quit(format!("{} Error: {}", StatusMark::Error, err), -1);
        }
    }

    println!("{} Done", StatusMark::Info);
}          