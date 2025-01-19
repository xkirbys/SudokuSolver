use clipboard::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(text: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(text.to_owned())?;

    Ok(())
}

pub fn read_from_clipboard() -> Result<String, Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    let contents = ctx.get_contents()?;

    Ok(contents)
}

pub fn key_pressed(input: &egui::InputState, key: egui::Key) -> bool {
    input.keys_down.contains(&key)
}