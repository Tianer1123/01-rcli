// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_sign,
    Base64SubCommand, Opts, SubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    // println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::Genpass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            rcli::TextSubCommand::Sign(opts) => match opts.format {
                rcli::TextSignFormat::Ed25519 => todo!(),
                rcli::TextSignFormat::Blake3 => {
                    process_text_sign(&opts.input, &opts.key, opts.format)?
                }
            },
            rcli::TextSubCommand::Verify(opts) => {
                println!("{:?}", opts);
            }
        },
    }

    Ok(())
}
