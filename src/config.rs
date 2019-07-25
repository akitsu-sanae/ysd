use buffer::BufferId;

#[derive(Clone, Debug)]
pub struct Config {
    pub mode_buffer_id: BufferId,
    pub msg_buffer_id: BufferId,
}
