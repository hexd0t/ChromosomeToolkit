#[repr(C)]
pub struct bSClockAdjustment {
    TicksPerSecond: f32,
    SecondsPerDay: u32,
    DaysPerYear: u32,
}

#[repr(C)]
pub struct bSTimeAndDate {
    Year: u32,
    Day: u32,
    Time: f32,
}
