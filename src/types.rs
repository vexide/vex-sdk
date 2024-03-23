#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GenericPositionData {
    pub position: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpticalData {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub brightness: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AdiExpanderData {
    pub adi_types: [AdiPortConfiguration; 8],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GpsData {
    pub offset_x: f64,
    pub offset_y: f64,

    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum AdiPortConfiguration {
    AnalogIn = 0,
    AnalogOut,
    DigitalIn,
    DigitalOut,

    SmartButton,
    SmartPot,

    LegacyButton,
    LegacyPotentiometer,
    LegacyLineSensor,
    LegacyLightSensor,
    LegacyGyro,
    LegacyAccelerometer,

    LegacyServo,
    LegacyPwm,

    QuadEncoder,
    Sonar,

    LegacyPwmSlew,

    Undefined = 255,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpticalRaw {
    pub clear: u16,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpticalRgb {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub brightness: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpticalGesture {
    pub udata: u8,
    pub ddata: u8,
    pub ldata: u8,
    pub rdata: u8,
    pub gesture_type: u8,
    pub padding: u8,
    pub count: u16,
    pub time: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum MagnetDuration {
    Short,
    Medium,
    Long,
    ExtraLong,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct GpsRaw {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct GpsAttitude {
    pitch: f64, // x
    roll: f64,  // y
    yaw: f64,   // z

    // spacial position on the field
    position_x: f64,
    position_y: f64,
    position_z: f64,

    // alternative roll, pitch and yaw
    az: f64,
    el: f64,
    rot: f64,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct GpsQuaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Image {
    width: u16,
    height: u16,
    data: *mut u32,
    p: *mut u32,
}