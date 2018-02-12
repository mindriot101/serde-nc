#[derive(Serialize)]
pub enum Instruction {
    EnableLoadMonitor,
    SetInchUnits,
    SetMaximumSpindleSpeed(i32),
}
