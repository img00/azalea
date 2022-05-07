use crate::mc_buf::{McBufReadable, McBufWritable, Readable, Writable};
use azalea_chat::component::Component;
use packet_macros::{GamePacket, McBufReadable, McBufWritable};
use std::io::{Read, Write};
use uuid::Uuid;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundPlayerInfoPacket {
    pub action: Action,
}

#[derive(Clone, Debug)]
pub enum Action {
    AddPlayer(Vec<AddPlayer>),
    UpdateGameMode(Vec<UpdateGameMode>),
    UpdateLatency(Vec<UpdateLatency>),
    UpdateDisplayName(Vec<UpdateDisplayName>),
    RemovePlayer(Vec<RemovePlayer>),
}

#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct PlayerProperty {
    name: String,
    value: String,
    signature: Option<String>,
}

#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct AddPlayer {
    uuid: Uuid,
    name: String,
    properties: Vec<PlayerProperty>,
    #[var]
    gamemode: u32,
    #[var]
    ping: i32,
    display_name: Option<Component>,
}

#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct UpdateGameMode {
    uuid: Uuid,
    #[var]
    gamemode: u32,
}

#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct UpdateLatency {
    uuid: Uuid,
    #[var]
    ping: i32,
}

#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct UpdateDisplayName {
    uuid: Uuid,
    display_name: Option<Component>,
}
#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct RemovePlayer {
    uuid: Uuid,
}

impl McBufReadable for Action {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let id = buf.read_byte()?;
        Ok(match id {
            0 => Action::AddPlayer(Vec::<AddPlayer>::read_into(buf)?),
            1 => Action::UpdateGameMode(Vec::<UpdateGameMode>::read_into(buf)?),
            2 => Action::UpdateLatency(Vec::<UpdateLatency>::read_into(buf)?),
            3 => Action::UpdateDisplayName(Vec::<UpdateDisplayName>::read_into(buf)?),
            4 => Action::RemovePlayer(Vec::<RemovePlayer>::read_into(buf)?),
            _ => panic!("Unknown player info action id: {}", id),
        })
    }
}
impl McBufWritable for Action {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_byte(match self {
            Action::AddPlayer(_) => 0,
            Action::UpdateGameMode(_) => 1,
            Action::UpdateLatency(_) => 2,
            Action::UpdateDisplayName(_) => 3,
            Action::RemovePlayer(_) => 4,
        })?;
        match self {
            Action::AddPlayer(players) => players.write_into(buf)?,
            Action::UpdateGameMode(players) => players.write_into(buf)?,
            Action::UpdateLatency(players) => players.write_into(buf)?,
            Action::UpdateDisplayName(players) => players.write_into(buf)?,
            Action::RemovePlayer(players) => players.write_into(buf)?,
        }
        Ok(())
    }
}