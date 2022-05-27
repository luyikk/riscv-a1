use anyhow::{Context, Result};
use structopt::StructOpt;
use std::path::PathBuf;
use async_process::{Command, Stdio};
use futures_lite::{io::BufReader, prelude::*};
use log::{LevelFilter};


#[derive(StructOpt, Debug)]
#[structopt(name = "qemu run")]
struct Opt{
    #[structopt(short, long, parse(from_os_str),default_value="qemu-system-riscv64")]
    qemu_path: PathBuf,
    #[structopt(short, long, parse(from_os_str),default_value="../bootloader/rustsbi-qemu.bin")]
    bootloader:PathBuf,
    #[structopt(short, long)]
    load_bin:String,
    #[structopt(short, long,default_value="2149580800")]
    addr:u64,
    #[structopt(short, long)]
    strip:bool
}


#[tokio::main]
async fn main()->Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Trace)
        .filter_module("mio::poll", LevelFilter::Error)
        .init();



    let opt:Opt = Opt::from_args();

    if opt.strip {
        let strip_out = Command::new("rust-objcopy")
            .arg("--strip-all")
            .arg(&opt.load_bin)
            .arg("-O")
            .arg("binary")
            .arg(format!("{}.bin", opt.load_bin)).output().await?;

        if strip_out.stdout.len() > 0 {
            println!("strip:{}", std::str::from_utf8(&strip_out.stdout)?);
        }

        if strip_out.stderr.len() > 0 {
            println!("strip err:{}", std::str::from_utf8(&strip_out.stderr)?);
        }
    }



    let mut command= Command::new(opt.qemu_path);
    command.arg("-machine")
        .arg("virt")
        .stdout(Stdio::piped())
        .arg("-nographic")
        .arg("-bios")
        .arg(opt.bootloader)
        .arg("-device")
        .arg(if opt.strip{
            format!("loader,file={}.bin,addr={:#0x}",opt.load_bin,opt.addr)
        }else{
            format!("loader,file={},addr={:#0x}",opt.load_bin,opt.addr)
        });

    println!("{:?}",command);

    let mut child= command.spawn()?;

    let mut lines = BufReader::new(child.stdout.take().context("not stdout")?).lines();
    while let Some(line) = lines.next().await {
        println!("{}", line?);
    }

    if child.stderr.is_some() {
        let mut lines = BufReader::new(child.stderr.take().context("not stderr")?).lines();
        while let Some(line) = lines.next().await {
            println!("{}", line?);
        }
    }

    Ok(())
}
