use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents the commands our key-value store can execute.
#[derive(Debug, PartialEq)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Invalid,
}

impl Command {
    /// Parses a command from a raw byte buffer.
    /// The protocol is simple and text-based, with parts separated by spaces.
    /// - SET key value\n
    /// - GET key\n
    pub fn from_buffer(buffer: &[u8]) -> Command {
        // Find the first line (ending in '\n') and convert it to a string.
        let request_str = match std::str::from_utf8(buffer).ok().and_then(|s| s.lines().next()) {
            Some(line) => line.trim(),
            None => return Command::Invalid,
        };

        let parts: Vec<&str> = request_str.split_whitespace().collect();
        match parts.get(0) {
            Some(&"SET") => {
                if parts.len() == 3 {
                    Command::Set {
                        key: parts[1].to_string(),
                        value: parts[2].to_string(),
                    }
                } else {
                    Command::Invalid
                }
            }
            Some(&"GET") => {
                if parts.len() == 2 {
                    Command::Get {
                        key: parts[1].to_string(),
                    }
                } else {
                    Command::Invalid
                }
            }
            _ => Command::Invalid,
        }
    }
}

/// The shared state of our key-value store.
///
/// We wrap the HashMap in a `tokio::sync::Mutex`. This kind of mutex is "async-aware",
/// meaning that if a task is waiting for the lock, it will yield control to the
/// Tokio runtime instead of blocking the whole thread. This allows other tasks to run.
///
/// The `Arc` (Atomically Referenced Counter) allows multiple tasks to safely share
/// ownership of the Mutex-protected HashMap.
pub type SharedState = Arc<Mutex<HashMap<String, String>>>;

/// Creates a new, empty shared state.
pub fn new_shared_state() -> SharedState {
    Arc::new(Mutex::new(HashMap::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Parser tests remain the same...
    #[test]
    fn test_parse_set_command() {
        let buffer = b"SET mykey myvalue\r\n";
        let command = Command::from_buffer(buffer);
        assert_eq!(
            command,
            Command::Set {
                key: "mykey".to_string(),
                value: "myvalue".to_string()
            }
        );
    }

    #[test]
    fn test_parse_get_command() {
        let buffer = b"GET mykey\n";
        let command = Command::from_buffer(buffer);
        assert_eq!(command, Command::Get { key: "mykey".to_string() });
    }

    #[tokio::test]
    async fn test_shared_state_async() {
        let state = new_shared_state();
        let state_clone = Arc::clone(&state);

        // Task 1: Set a value
        let handle1 = tokio::spawn(async move {
            let mut db = state.lock().await;
            db.insert("hello".to_string(), "world".to_string());
        });

        // Task 2: Get a value
        let handle2 = tokio::spawn(async move {
            // Wait for task 1 to complete by yielding control briefly.
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            let db = state_clone.lock().await;
            db.get("hello").cloned()
        });

        handle1.await.unwrap();
        let result = handle2.await.unwrap();

        assert_eq!(result, Some("world".to_string()));
    }
}
