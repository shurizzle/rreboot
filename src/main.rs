use byteorder::{LittleEndian, ReadBytesExt};
use efivar::efi::{Variable, VariableFlags};

fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = efivar::system();
    let active = manager
        .read(&Variable::new("BootCurrent"))
        .unwrap()
        .0
        .as_slice()
        .read_u16::<LittleEndian>()
        .unwrap();
    manager.write(&Variable::new("BootNext"), VariableFlags::default(), &active.to_le_bytes())?;
    system_shutdown::reboot()?;
    Ok(())
}

#[cfg(not(debug_assertions))]
fn main() {
    if let Err(err) = _main() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

#[cfg(debug_assertions)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    _main()
}
