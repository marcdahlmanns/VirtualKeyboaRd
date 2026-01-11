#[cfg(not(test))]
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InputError {
    #[error("Failed to simulate key press: {0}")]
    KeyPressError(String),
}

#[derive(Clone)]
pub struct InputSimulator {
    os_type: OSType,
    #[cfg(test)]
    test_file: Option<String>,
}

#[derive(Clone)]
enum OSType {
    Linux,
    MacOS,
}

impl InputSimulator {
    pub fn new() -> Self {
        let os_type = if cfg!(target_os = "macos") {
            OSType::MacOS
        } else {
            OSType::Linux
        };
        Self { 
            os_type,
            #[cfg(test)]
            test_file: None,
        }
    }

    #[cfg(test)]
    pub fn set_test_file(&mut self, file_path: String) {
        self.test_file = Some(file_path);
    }

    pub fn simulate_key_press(&self, key: char) -> Result<(), InputError> {
        #[cfg(test)]
        {
            if let Some(file_path) = &self.test_file {
                use std::fs::OpenOptions;
                use std::io::Write;
                
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(file_path)
                    .map_err(|e| InputError::KeyPressError(format!("Failed to open test file: {}", e)))?;
                
                file.write_all(&[key as u8])
                    .map_err(|e| InputError::KeyPressError(format!("Failed to write to test file: {}", e)))?;
                Ok(())
            } else {
                Ok(())
            }
        }

        #[cfg(not(test))]
        {
            match self.os_type {
                OSType::Linux => {
                    let output = Command::new("wtype")
                        .arg(key.to_string())
                        .output()
                        .map_err(|e| InputError::KeyPressError(format!("wtype error: {}", e)))?;

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(InputError::KeyPressError(format!(
                            "wtype failed: {}",
                            stderr
                        )));
                    }
                }
                OSType::MacOS => {
                    // First ensure TextEdit is running and frontmost
                    let activate_script = r#"
                        tell application "TextEdit"
                            if not running then
                                activate
                                delay 0.5
                            end if
                            activate
                            delay 0.1
                        end tell
                    "#;

                    let output = Command::new("osascript")
                        .arg("-e")
                        .arg(activate_script)
                        .output()
                        .map_err(|e| InputError::KeyPressError(format!("AppleScript error: {}", e)))?;

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(InputError::KeyPressError(format!(
                            "Failed to activate TextEdit: {}",
                            stderr
                        )));
                    }

                    // Then type the character
                    let type_script = format!(
                        r#"
                        tell application "System Events"
                            tell process "TextEdit"
                                set frontmost to true
                                keystroke "{}"
                            end tell
                        end tell
                        "#,
                        key
                    );

                    let output = Command::new("osascript")
                        .arg("-e")
                        .arg(type_script)
                        .output()
                        .map_err(|e| InputError::KeyPressError(format!("AppleScript error: {}", e)))?;

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(InputError::KeyPressError(format!(
                            "Failed to type character: {}",
                            stderr
                        )));
                    }
                }
            }
            Ok(())
        }
    }

    pub fn simulate_space(&self) -> Result<(), InputError> {
        #[cfg(test)]
        {
            if let Some(file_path) = &self.test_file {
                use std::fs::OpenOptions;
                use std::io::Write;
                
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(file_path)
                    .map_err(|e| InputError::KeyPressError(format!("Failed to open test file: {}", e)))?;
                
                file.write_all(&[b' '])
                    .map_err(|e| InputError::KeyPressError(format!("Failed to write to test file: {}", e)))?;
                Ok(())
            } else {
                Ok(())
            }
        }

        #[cfg(not(test))]
        match self.os_type {
            OSType::Linux => self.simulate_key_press(' '),
            OSType::MacOS => {
                // First ensure TextEdit is running and frontmost
                let activate_script = r#"
                    tell application "TextEdit"
                        if not running then
                            activate
                            delay 0.5
                        end if
                        activate
                        delay 0.1
                    end tell
                "#;

                let output = Command::new("osascript")
                    .arg("-e")
                    .arg(activate_script)
                    .output()
                    .map_err(|e| InputError::KeyPressError(format!("AppleScript error: {}", e)))?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(InputError::KeyPressError(format!(
                        "Failed to activate TextEdit: {}",
                        stderr
                    )));
                }

                // Then type the space
                let type_script = r#"
                    tell application "System Events"
                        tell process "TextEdit"
                            set frontmost to true
                            keystroke space
                        end tell
                    end tell
                "#;

                let output = Command::new("osascript")
                    .arg("-e")
                    .arg(type_script)
                    .output()
                    .map_err(|e| InputError::KeyPressError(format!("AppleScript error: {}", e)))?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(InputError::KeyPressError(format!(
                        "Failed to type space: {}",
                        stderr
                    )));
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_simulator(file_name: &str) -> (InputSimulator, String) {
        let test_file = format!("test_{}.txt", file_name);
        let mut simulator = InputSimulator::new();
        simulator.set_test_file(test_file.clone());
        (simulator, test_file)
    }

    fn cleanup(file_path: &str) {
        if let Err(e) = fs::remove_file(file_path) {
            eprintln!("Warning: Failed to remove test file {}: {}", file_path, e);
        }
    }

    #[test]
    fn test_keyboard_input() {
        let (simulator, test_file) = setup_simulator("basic");

        // Simulate typing "test"
        simulator.simulate_key_press('t').unwrap();
        simulator.simulate_key_press('e').unwrap();
        simulator.simulate_key_press('s').unwrap();
        simulator.simulate_key_press('t').unwrap();

        // Read the file and verify contents
        let contents = fs::read_to_string(&test_file)
            .expect("Failed to read test file");
        
        assert_eq!(contents, "test");

        cleanup(&test_file);
    }

    #[test]
    fn test_keyboard_input_with_space() {
        let (simulator, test_file) = setup_simulator("space");

        // Simulate typing "test test"
        simulator.simulate_key_press('t').unwrap();
        simulator.simulate_key_press('e').unwrap();
        simulator.simulate_key_press('s').unwrap();
        simulator.simulate_key_press('t').unwrap();
        simulator.simulate_space().unwrap();
        simulator.simulate_key_press('t').unwrap();
        simulator.simulate_key_press('e').unwrap();
        simulator.simulate_key_press('s').unwrap();
        simulator.simulate_key_press('t').unwrap();

        // Read the file and verify contents
        let contents = fs::read_to_string(&test_file)
            .expect("Failed to read test file");
        
        assert_eq!(contents, "test test");

        cleanup(&test_file);
    }

    #[test]
    fn test_multiple_consecutive_spaces() {
        let (simulator, test_file) = setup_simulator("multi_space");

        // Type "hi  world" with two spaces
        simulator.simulate_key_press('h').unwrap();
        simulator.simulate_key_press('i').unwrap();
        simulator.simulate_space().unwrap();
        simulator.simulate_space().unwrap();
        simulator.simulate_key_press('w').unwrap();
        simulator.simulate_key_press('o').unwrap();
        simulator.simulate_key_press('r').unwrap();
        simulator.simulate_key_press('l').unwrap();
        simulator.simulate_key_press('d').unwrap();

        let contents = fs::read_to_string(&test_file)
            .expect("Failed to read test file");
        
        assert_eq!(contents, "hi  world");

        cleanup(&test_file);
    }

    #[test]
    fn test_special_characters() {
        let (simulator, test_file) = setup_simulator("special");

        // Test special characters
        let special_chars = "!@#$%^&*()";
        for c in special_chars.chars() {
            simulator.simulate_key_press(c).unwrap();
        }

        let contents = fs::read_to_string(&test_file)
            .expect("Failed to read test file");
        
        assert_eq!(contents, special_chars);

        cleanup(&test_file);
    }

    #[test]
    fn test_long_input() {
        let (simulator, test_file) = setup_simulator("long");

        // Generate a long string with repeated pattern
        let pattern = "The quick brown fox jumps over the lazy dog. ";
        let repetitions = 5;
        let expected = pattern.repeat(repetitions);

        // Type the long string
        for c in expected.chars() {
            if c == ' ' {
                simulator.simulate_space().unwrap();
            } else {
                simulator.simulate_key_press(c).unwrap();
            }
        }

        let contents = fs::read_to_string(&test_file)
            .expect("Failed to read test file");
        
        assert_eq!(contents, expected);

        cleanup(&test_file);
    }

    #[test]
    fn test_file_handling_error() {
        let mut simulator = InputSimulator::new();
        // Set an invalid file path
        simulator.set_test_file("/invalid/path/that/should/not/exist/test.txt".to_string());

        // Attempt to write should result in an error
        let result = simulator.simulate_key_press('a');
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("Failed to open test file"));
        }
    }
} 