use nesmc_emu::NesMachine;
use rfd::FileDialog;

pub fn open_rom_dialog(machine: &mut NesMachine) {
    if let Some(path) = FileDialog::new()
        .add_filter("NES file", &["nes"])
        .pick_file()
    {
        let _ = machine.open(path);
    }
}
