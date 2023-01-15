use crate::cli_args::*;
use cargo::{
    core::{
        compiler::{CompileKind, CompileTarget},
        FeatureValue, Workspace,
    },
    ops::{CompileOptions, Packages},
    util::command_prelude::CompileMode,
};
use std::{
    collections::BTreeSet, convert::TryFrom, fs::File, io::Seek, path::PathBuf, rc::Rc,
    time::Instant,
};
use std::{fs, io, path::Path};

pub fn build(workspace: &mut Workspace, args: BuildArgs) -> std::io::Result<PathBuf> {
    match args.arch {
        Arch::X86_64 => build_x86_64(workspace, args),
        Arch::Aarch64 => unimplemented!(),
    }
}

fn build_x86_64(workspace: &mut Workspace, args: BuildArgs) -> std::io::Result<PathBuf> {
    let compile_start = Instant::now();
    let mut options = CompileOptions::new(workspace.config(), CompileMode::Build)
        .expect("Could not get compile options");
    options.build_config.requested_kinds = vec![CompileKind::Target(
        CompileTarget::new("x86_64-unknown-uefi").unwrap(),
    )];
    options.spec = Packages::Packages(vec!["tkern_arch_x86_64".to_string()]);
    if args.with_qemu {
        options.cli_features.features =
            Rc::new(BTreeSet::from([FeatureValue::Feature("qemu".into())]));
    }

    let result = cargo::ops::compile(workspace, &options).expect("Could not compile");
    let output = result.binaries.first().expect("No binary created");
    println!("In {}:", output.path.parent().unwrap().to_string_lossy());
    println!(
        "  Kernel compiled to {} ({:?})",
        output.path.file_name().unwrap().to_string_lossy(),
        compile_start.elapsed(),
    );
    let fat_path = output.path.with_extension("fat");
    let disk_path = output.path.with_extension("gdt");
    let fat_start = Instant::now();
    create_fat_filesystem(&fat_path, &output.path);
    let fat_elapsed = fat_start.elapsed();
    let gpt_start = Instant::now();
    create_gpt_disk(&disk_path, &fat_path);
    let gpt_elapsed = gpt_start.elapsed();

    println!(
        "  Generated {} ({:?})",
        fat_path.file_name().unwrap().to_string_lossy(),
        fat_elapsed
    );
    println!(
        "  Generated {} ({:?})",
        disk_path.file_name().unwrap().to_string_lossy(),
        gpt_elapsed
    );
    Ok(output.path.clone())
}

fn create_fat_filesystem(fat_path: &Path, efi_file: &Path) {
    // retrieve size of `.efi` file and round it up
    let efi_size = fs::metadata(efi_file).unwrap().len();
    // size of a megabyte
    let mb = 1024 * 1024;
    // round it to next megabyte
    let efi_size_rounded = ((efi_size - 1) / mb + 1) * mb;

    // create new filesystem image file at the given path and set its length
    let fat_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(fat_path)
        .unwrap();
    fat_file.set_len(efi_size_rounded).unwrap();

    // create new FAT file system and open it
    let format_options = fatfs::FormatVolumeOptions::new();
    fatfs::format_volume(&fat_file, format_options).unwrap();
    let filesystem = fatfs::FileSystem::new(&fat_file, fatfs::FsOptions::new()).unwrap();

    // copy EFI file to FAT filesystem
    let root_dir = filesystem.root_dir();
    root_dir.create_dir("efi").unwrap();
    root_dir.create_dir("efi/boot").unwrap();
    let mut bootx64 = root_dir.create_file("efi/boot/bootx64.efi").unwrap();
    bootx64.truncate().unwrap();
    io::copy(&mut fs::File::open(efi_file).unwrap(), &mut bootx64).unwrap();
}

fn create_gpt_disk(disk_path: &Path, fat_image: &Path) {
    // create new file
    let mut disk = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(disk_path)
        .unwrap();

    // set file size
    let partition_size: u64 = fs::metadata(fat_image).unwrap().len();
    let disk_size = partition_size + 1024 * 64; // for GPT headers
    disk.set_len(disk_size).unwrap();

    // create a protective MBR at LBA0 so that disk is not considered
    // unformatted on BIOS systems
    let mbr = gpt::mbr::ProtectiveMBR::with_lb_size(
        u32::try_from((disk_size / 512) - 1).unwrap_or(0xFF_FF_FF_FF),
    );
    mbr.overwrite_lba0(&mut disk).unwrap();

    // create new GPT structure
    let block_size = gpt::disk::LogicalBlockSize::Lb512;
    let mut gpt = gpt::GptConfig::new()
        .writable(true)
        .initialized(false)
        .logical_block_size(block_size)
        .create_from_device(Box::new(&mut disk), None)
        .unwrap();
    gpt.update_partitions(Default::default()).unwrap();

    // add new EFI system partition and get its byte offset in the file
    let partition_id = gpt
        .add_partition("boot", partition_size, gpt::partition_types::EFI, 0, None)
        .unwrap();
    let partition = gpt.partitions().get(&partition_id).unwrap();
    let start_offset = partition.bytes_start(block_size).unwrap();

    // close the GPT structure and write out changes
    gpt.write().unwrap();

    // place the FAT filesystem in the newly created partition
    disk.seek(io::SeekFrom::Start(start_offset)).unwrap();
    io::copy(&mut File::open(fat_image).unwrap(), &mut disk).unwrap();
}
