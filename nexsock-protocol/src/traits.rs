use crate::commands::Command;

pub trait ServiceCommand {
    type Input;
    type Output;

    const COMMAND: Command;

    fn into_payload(self) -> Self::Input;
}

#[cfg(debug_assertions)]
pub trait PayloadDebug {
    fn debug_dump(&self) -> String;
}

#[cfg(debug_assertions)]
impl PayloadDebug for [u8] {
    fn debug_dump(&self) -> String {
        use std::fmt::Write;
        let mut s = String::new();
        write!(s, "Payload ({} bytes): [", self.len()).unwrap();
        for (i, b) in self.iter().enumerate() {
            if i > 0 {
                write!(s, ", ").unwrap();
            }
            write!(s, "{:#04x}", b).unwrap();
        }
        write!(s, "]").unwrap();
        s
    }
}
