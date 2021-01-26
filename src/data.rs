//! Virtual machine primitive datas.

use std::{mem, ops, ptr::null};

/// Virtual machine command payload.
#[derive(Debug)]
#[repr(C)]
pub struct BytesBuffer {
    /// Size of the data.
    pub size: u64,
    /// Base address of the data.
    pub base: *const u8,
}

/// Virtual machine command.
#[derive(Debug)]
#[repr(C)]
pub struct Command {
    /// Unique id of the command.
    pub id: u64,
    /// The data that holds the command.
    pub payload: BytesBuffer,
}

/// Commands array.
#[repr(C)]
pub struct Commands {
    /// Address to commands region
    pub commands: *const Command,
    /// Count of commands.
    pub size: usize,
}

impl Command {
    /// Create a new command with a given payload.
    pub fn new(id: u64, payload: BytesBuffer) -> Self {
        Command { id, payload }
    }

    /// Create a command without a payload.
    pub fn empty(id: u64) -> Self {
        Command {
            id,
            payload: BytesBuffer::empty(),
        }
    }
}

impl Commands {
    /// Create a commands instance without commands.
    pub fn empty() -> Self {
        Commands {
            size: 0,
            commands: null::<Command>(),
        }
    }
}

// pub enum CommandPayloadItem {
//     Int32(i32),
//     Int64(i64),
//     Vec2f(Vec2f),
//     Vec4f(Vec4f),
// }

// union CommandPayloadItemC {
//     int32: i32,
//     int64: i64,
//     vec2f: Vec2f,
//     vec4f: Vec4f,
// }

// struct CommandPayload {
//     data: Vec<CommandPayloadItemC>,
// }

// impl CommandPayload {
//     pub fn new(items: &[CommandPayloadItem]) -> Self {
//         for item in items.iter() {
//         }

//         todo!()
//     }
// }

impl BytesBuffer {
    /// Create a new payload with a given array.
    pub fn new<T>(values: &[T]) -> Self {
        let base = values
            .first()
            .map(|value| value as *const T)
            .unwrap_or(null::<T>());

        BytesBuffer {
            size: (mem::size_of::<T>() as u64) * (values.len() as u64),
            base: base as *const u8,
        }
    }

    /// Create a new payload with a given array.
    // pub fn new_command_payload(_values: &[CommandPayloadItem]) -> Self {
        // todo!()
    // }

    /// Create empty payload without any data.
    pub fn empty() -> Self {
        BytesBuffer {
            size: 0,
            base: null::<u8>(),
        }
    }
}
