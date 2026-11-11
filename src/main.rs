use caps_widget::LockState;

fn main() {
    // Placeholder until the Win32 adapter lands: report the default state.
    println!("{}", LockState::default().tooltip());
}
