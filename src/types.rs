#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct ImuRaw {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ImuOrientationMode {
    ZUp = 0x00,
    ZDown = 0x10,
    XUp = 0x20,
    XDown = 0x30,
    YUp = 0x40,
    YDown = 0x50,
    Auto = 0x80,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImuData {
    pub orientation: ImuOrientationMode,
    pub rotation: ImuRaw,
    pub acceleration: ImuRaw,
    pub reset_timestamp: u32,
}

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
pub enum ControllerId {
    Primary = 0,
    Partner = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ControllerChannel {
    JoystickLeftX = 0,
    JoystickLeftY,
    JoystickRightX,
    JoystickRightY,
    AnalogSpare1,
    AnalogSpare2,

    ButtonL1,
    ButtonL2,
    ButtonR1,
    ButtonR2,
    ButtonUp,
    ButtonDown,
    ButtonLeft,
    ButtonRight,
    ButtonX,
    ButtonB,
    ButtonY,
    ButtonA,

    ButtonSEL,

    BatteryLevel,

    ButtonAll,
    Flags,
    BatteryCapacity,
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
pub enum VisionMode {
    Normal = 0,
    Mixed = 1,
    LineDetect = 2,
    Test = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum VisionBlockType {
    Normal = 0,
    ColorCode = 1,
    LineDetect = 2,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct VisionObject {
    /// block signature
    pub signature: u16,
    /// block type
    pub block_type: VisionBlockType,
    /// left side of block
    pub xoffset: u16,
    /// top of block
    pub yoffset: u16,
    /// width of block
    pub width: u16,
    /// height of block
    pub height: u16,
    /// angle of CC block in 0.1 deg units
    pub angle: u16,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct VisionSignature {
    pub id: u8,
    pub flags: u8,
    pub pad: [u8; 2],
    pub range: f32,
    pub u_min: i32,
    pub u_max: i32,
    pub u_mean: i32,
    pub v_min: i32,
    pub v_max: i32,
    pub v_mean: i32,
    pub m_rgb: u32,
    pub m_type: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum VisionWhiteBalanceMode {
    Normal = 0,
    Start = 1,
    Manual = 2,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct VisionRgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub brightness: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum VisionLedMode {
    Auto = 0,
    Manual = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum VisionWifiMode {
    Off = 0,
    On = 1,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct ImuQuaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct ImuAttitude {
    pub pitch: f64,
    pub roll: f64,
    pub yaw: f64,
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