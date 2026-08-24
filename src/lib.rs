//! JavaScript/WASM interface for MathCAT.
//!
//! This crate wraps MathCAT's public Rust interface (`libmathcat`) with a Diplomat bridge for
//! consumption from JavaScript/WASM. MathCAT tracks state in thread-locals rather than through a
//! caller-held handle, so `MathCat` exists only as a namespace of static methods
//! (`MathCat.setMathml(...)` in JS).

#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::DiplomatWrite;
    use libmathcat::interface;
    use std::fmt::Write as _;

    #[diplomat::opaque]
    pub struct MathCat;

    /// A MathCAT error, exposed as an opaque handle so its message can be pulled out on demand.
    #[diplomat::opaque]
    pub struct MathCatError(String);

    impl MathCatError {
        pub fn message_write(&self, write: &mut DiplomatWrite) {
            let _ = write.write_str(&self.0);
            write.flush();
        }
    }

    fn to_ffi_error(e: libmathcat::errors::Error) -> Box<MathCatError> {
        Box::new(MathCatError(interface::errors_to_string(&e)))
    }

    /// A list of strings, for the handful of functions that return `Vec<String>`.
    #[diplomat::opaque]
    pub struct MathCatStringList(Vec<String>);

    impl MathCatStringList {
        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        /// Writes the string at `index` into `write`; returns `false` if `index` is out of range.
        pub fn get(&self, index: usize, write: &mut DiplomatWrite) -> bool {
            match self.0.get(index) {
                Some(s) => {
                    let _ = write.write_str(s);
                    write.flush();
                    true
                }
                None => false,
            }
        }
    }

    /// The start/end braille cell positions returned by `get_braille_position`.
    pub struct BraillePosition {
        pub start: usize,
        pub end: usize,
    }

    impl MathCat {
        pub fn set_rules_dir(dir: &str) -> Result<(), Box<MathCatError>> {
            interface::set_rules_dir(dir).map_err(to_ffi_error)
        }

        pub fn get_version(write: &mut DiplomatWrite) {
            let _ = write.write_str(&interface::get_version());
            write.flush();
        }

        pub fn set_mathml(mathml_str: &str, write: &mut DiplomatWrite) -> Result<(), Box<MathCatError>> {
            let cleaned = interface::set_mathml(mathml_str).map_err(to_ffi_error)?;
            let _ = write.write_str(&cleaned);
            write.flush();
            Ok(())
        }

        pub fn get_spoken_text(write: &mut DiplomatWrite) -> Result<(), Box<MathCatError>> {
            let text = interface::get_spoken_text().map_err(to_ffi_error)?;
            let _ = write.write_str(&text);
            write.flush();
            Ok(())
        }

        pub fn get_overview_text(write: &mut DiplomatWrite) -> Result<(), Box<MathCatError>> {
            let text = interface::get_overview_text().map_err(to_ffi_error)?;
            let _ = write.write_str(&text);
            write.flush();
            Ok(())
        }

        pub fn get_preference(name: &str, write: &mut DiplomatWrite) -> Result<(), Box<MathCatError>> {
            let value = interface::get_preference(name).map_err(to_ffi_error)?;
            let _ = write.write_str(&value);
            write.flush();
            Ok(())
        }

        pub fn set_preference(name: &str, value: &str) -> Result<(), Box<MathCatError>> {
            interface::set_preference(name, value).map_err(to_ffi_error)
        }

        pub fn get_braille(nav_node_id: &str, write: &mut DiplomatWrite) -> Result<(), Box<MathCatError>> {
            let braille = interface::get_braille(nav_node_id).map_err(to_ffi_error)?;
            let _ = write.write_str(&braille);
            write.flush();
            Ok(())
        }

        pub fn get_navigation_braille(write: &mut DiplomatWrite) -> Result<(), Box<MathCatError>> {
            let braille = interface::get_navigation_braille().map_err(to_ffi_error)?;
            let _ = write.write_str(&braille);
            write.flush();
            Ok(())
        }

        pub fn do_navigate_keypress(
            key: usize,
            shift_key: bool,
            control_key: bool,
            alt_key: bool,
            meta_key: bool,
            write: &mut DiplomatWrite,
        ) -> Result<(), Box<MathCatError>> {
            let speech = interface::do_navigate_keypress(key, shift_key, control_key, alt_key, meta_key)
                .map_err(to_ffi_error)?;
            let _ = write.write_str(&speech);
            write.flush();
            Ok(())
        }

        pub fn do_navigate_command(command: &str, write: &mut DiplomatWrite) -> Result<(), Box<MathCatError>> {
            let speech = interface::do_navigate_command(command).map_err(to_ffi_error)?;
            let _ = write.write_str(&speech);
            write.flush();
            Ok(())
        }

        pub fn set_navigation_node(id: &str, offset: usize) -> Result<(), Box<MathCatError>> {
            interface::set_navigation_node(id, offset).map_err(to_ffi_error)
        }

        pub fn get_navigation_mathml(write: &mut DiplomatWrite) -> Result<usize, Box<MathCatError>> {
            let (mathml, offset) = interface::get_navigation_mathml().map_err(to_ffi_error)?;
            let _ = write.write_str(&mathml);
            write.flush();
            Ok(offset)
        }

        pub fn get_navigation_mathml_id(write: &mut DiplomatWrite) -> Result<usize, Box<MathCatError>> {
            let (id, offset) = interface::get_navigation_mathml_id().map_err(to_ffi_error)?;
            let _ = write.write_str(&id);
            write.flush();
            Ok(offset)
        }

        pub fn get_braille_position() -> Result<BraillePosition, Box<MathCatError>> {
            let (start, end) = interface::get_braille_position().map_err(to_ffi_error)?;
            Ok(BraillePosition { start, end })
        }

        pub fn get_navigation_node_from_braille_position(
            position: usize,
            write: &mut DiplomatWrite,
        ) -> Result<usize, Box<MathCatError>> {
            let (id, offset) =
                interface::get_navigation_node_from_braille_position(position).map_err(to_ffi_error)?;
            let _ = write.write_str(&id);
            write.flush();
            Ok(offset)
        }

        pub fn get_supported_braille_codes() -> Result<Box<MathCatStringList>, Box<MathCatError>> {
            interface::get_supported_braille_codes()
                .map(|v| Box::new(MathCatStringList(v)))
                .map_err(to_ffi_error)
        }

        pub fn get_supported_languages() -> Result<Box<MathCatStringList>, Box<MathCatError>> {
            interface::get_supported_languages()
                .map(|v| Box::new(MathCatStringList(v)))
                .map_err(to_ffi_error)
        }

        pub fn get_supported_speech_styles(lang: &str) -> Result<Box<MathCatStringList>, Box<MathCatError>> {
            interface::get_supported_speech_styles(lang)
                .map(|v| Box::new(MathCatStringList(v)))
                .map_err(to_ffi_error)
        }
    }
}
