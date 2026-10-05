use std::sync::OnceLock;

#[derive(Clone, Copy)]
pub(super) struct TalkieBoxLayout {
    pub length: usize,
    pub offsets: [usize; 5],
}

pub(super) fn layout(id: u16, packetver: u32) -> Option<TalkieBoxLayout> {
    type PacketRange = (u32, u32, u16, usize, [usize; 5]);
    static PACKETS: OnceLock<Vec<PacketRange>> = OnceLock::new();
    // Packet IDs are reused for unrelated requests across client versions.
    PACKETS
        .get_or_init(|| serde_json::from_str(include_str!("talkie_box_packets.json")).expect("Embedded Talkie Box layouts are invalid"))
        .iter()
        .find(|(start, end, header, ..)| *header == id && (*start..*end).contains(&packetver))
        .map(|(_, _, _, length, offsets)| TalkieBoxLayout {
            length: *length,
            offsets: *offsets,
        })
}
