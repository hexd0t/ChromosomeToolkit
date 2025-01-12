use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// gEDirection
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Direction {
    /// eEVelocityDirectionFrom_None
    None = 0x00000000,
    /// eEVelocityDirectionFrom_StartPositionAndOwner
    StartPositionAndOwner = 0x00000001,
    /// eEVelocityDirectionFrom_OwnerAndStartPosition
    OwnerAndStartPosition = 0x00000002,
    /// eEVelocityDirectionFrom_Owner
    Owner = 0x00000003,
    /// eEVelocityDirectionFrom_World
    World = 0x00000004,
}

// eCVegetationBrush_PS.ColorNoiseTurbulence
// eCVegetationBrush_PS.ProbabilityNoiseTurbulence
/// bENoiseTurbulence
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum NoiseTurbulence {
    /// bETurbulence_FractalSum   
    FractalSum = 0x00000000,
    /// bETurbulence_FractalAbsSum
    FractalAbsSum = 0x00000001,
}

// eCImageFilterRTBase.ColorFormat
/// eCImageFilterRTBase_eCGfxShared_eEColorFormat
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ImageFilterRTColorFormat {
    /// eEColorFormat_Unknown      
    Unknown = 0x00000000,
    /// eEColorFormat_A8R8G8B8     
    A8R8G8B8 = 0x00000015,
    /// eEColorFormat_X1R5G5B5     
    X1R5G5B5 = 0x00000018,
    /// eEColorFormat_A1R5G5B5     
    A1R5G5B5 = 0x00000019,
    /// eEColorFormat_A4R4G4B4     
    A4R4G4B4 = 0x0000001A,
    /// eEColorFormat_R16F         
    R16F = 0x0000006F,
    /// eEColorFormat_G16R16F      
    G16R16F = 0x00000070,
    /// eEColorFormat_A16B16G16R16F
    A16B16G16R16F = 0x00000071,
    /// eEColorFormat_R32F         
    R32F = 0x00000072,
    /// eEColorFormat_G32R32F      
    G32R32F = 0x00000073,
    /// eEColorFormat_A32B32G32R32F
    A32B32G32R32F = 0x00000074,
}

// eCImageResource2.PixelFormat
/// eCImageResource2_eCGfxShared_eEColorFormat
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ImageResource2ColorFormat {
    /// eEColorFormat_Unknown
    Unknown = 0x00000000,
    /// eEColorFormat_A8R8G8B8
    A8R8G8B8 = 0x00000015,
    /// eEColorFormat_X8R8G8B8
    X8R8G8B8 = 0x00000016,
    /// eEColorFormat_DXT1    
    DXT1 = 0x31545844,
    /// eEColorFormat_DXT2    
    DXT2 = 0x32545844,
    /// eEColorFormat_DXT3    
    DXT3 = 0x33545844,
    /// eEColorFormat_DXT4    
    DXT4 = 0x34545844,
    /// eEColorFormat_DXT5    
    DXT5 = 0x35545844,
}

// eCGuiWindow2.AnchorMode
/// eEAnchorMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum AnchorMode {
    /// eEAnchorMode_Default
    Default = 0x00000000,
    /// eEAnchorMode_Relative
    Relative = 0x00000001,
}

// eCAudioEmitter_PS.FallOff
// eCAudioRollOffPreset.FallOff
// gCEffectCommandPlaySound.FallOff
// gCEffectCommandPlayVoice.FallOff
/// eEAudioChannelFallOff
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum AudioChannelFallOff {
    /// eEAudioChannelFallOff_Logarithmic
    Logarithmic = 0x00000000,
    /// eEAudioChannelFallOff_Linear     
    Linear = 0x00000001,
}

// gCAudioVolumeScrollBar2.AudioChannelGroup
// gCAudioVolumeTrackbar2.AudioChannelGroup
/// eEAudioChannelGroup
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum AudioChannelGroup {
    /// eEAudioChannelGroup_Master
    Master = 0x00000000,
    /// eEAudioChannelGroup_Voice  
    Voice = 0x00000001,
    /// eEAudioChannelGroup_Music  
    Music = 0x00000003,
    /// eEAudioChannelGroup_FX     
    FX = 0x00000004,
    /// eEAudioChannelGroup_Ambient
    Ambient = 0x00000007,
}

// eCAudioEmitter_PS.SpawningMode
/// eEAudioEmitterMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum AudioEmitterMode {
    /// eEAudioEmitterMode_Once  
    Once = 0x00000000,
    /// eEAudioEmitterMode_Loop  
    Loop = 0x00000001,
    /// eEAudioEmitterMode_Repeat
    Repeat = 0x00000002,
}

// eCAudioEmitter_PS.Shape
/// eEAudioEmitterShape
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum AudioEmitterShape {
    /// eEAudioEmitterShape_Point
    Point = 0x00000000,
    /// eEAudioEmitterShape_Box  
    Box = 0x00000001,
}

// eCBillboard_PS.TargetMode
/// eEBillboardTargetMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum BillboardTargetMode {
    /// eEBillboardTargetMode_Self
    #[serde(rename = "Self")]
    _Self = 0x00000000,
    /// eEBillboardTargetMode_Parent
    Parent = 0x00000001,
    /// eEBillboardTargetMode_Target
    Target = 0x00000002,
}

// eCIlluminated_PS.CastDirLightShadowsOverwrite
// eCIlluminated_PS.CastPntLightShadowsOverwrite
// eCIlluminated_PS.CastStaticShadowsOverwrite
/// eEBoolOverwrite
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum BoolOverwrite {
    /// eEBoolOverwrite_None
    None = 0x00000000,
    /// eEBoolOverwrite_True
    True = 0x00000001,
    /// eEBoolOverwrite_False
    False = 0x00000002,
}

// eCGuiRadioButton2.CheckState
/// eCGuiRadioButton2_eECheckState
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum RadioCheckState {
    /// eECheckState_Unchecked
    Unchecked = 0x00000000,
    /// eECheckState_Checked  
    Checked = 0x00000001,
}

// eCGuiCheckBox2.CheckState
/// eCGuiCheckBox2_eECheckState
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CheckBoxCheckState {
    /// eECheckState_Unchecked    
    Unchecked = 0x00000000,
    /// eECheckState_Checked      
    Checked = 0x00000001,
    /// eECheckState_Indeterminate
    Indeterminate = 0x00000002,
}

// eCCollisionShape_PS.Group
/// eECollisionGroup
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CollisionGroup {
    /// eECollisionGroup_Static          
    Static = 0x00000001,
    /// eECollisionGroup_Dynamic         
    Dynamic = 0x00000002,
    /// eECollisionGroup_Player          
    Player = 0x00000003,
    /// eECollisionGroup_NPC             
    NPC = 0x00000004,
    /// eECollisionGroup_Item_Equipped   
    Equipped = 0x00000005,
    /// eECollisionGroup_Item_World      
    World = 0x00000006,
    /// eECollisionGroup_Item_Attack     
    Attack = 0x00000007,
    /// eECollisionGroup_Interactable    
    Interactable = 0x00000008,
    /// eECollisionGroup_Trigger         
    Trigger = 0x00000009,
    /// eECollisionGroup_Zone            
    Zone = 0x0000000A,
    /// eECollisionGroup_Camera          
    Camera = 0x0000000B,
    /// eECollisionGroup_Tree            
    Tree = 0x0000000C,
    /// eECollisionGroup_DownCharacter   
    DownCharacter = 0x0000000D,
    /// eECollisionGroup_PlayerTrigger   
    PlayerTrigger = 0x0000000E,
    /// eECollisionGroup_ObjectTrigger   
    ObjectTrigger = 0x0000000F,
    /// eECollisionGroup_Ghost           
    Ghost = 0x00000010,
    /// eECollisionGroup_Mover           
    Mover = 0x00000011,
    /// eECollisionGroup_RagDoll         
    RagDoll = 0x00000012,
    /// eECollisionGroup_CharacterTrigger
    CharacterTrigger = 0x00000013,
}

// eCCollisionShape.ShapeType
/// eECollisionShapeType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CollisionShapeType {
    /// eECollisionShapeType_Box       
    Box = 0x00000003,
    /// eECollisionShapeType_TriMesh   
    TriMesh = 0x00000001,
    /// eECollisionShapeType_ConvexHull
    ConvexHull = 0x00000006,
    /// eECollisionShapeType_None      
    None = 0x00000000,
    /// eECollisionShapeType_Plane     
    Plane = 0x00000002,
    /// eECollisionShapeType_Capsule   
    Capsule = 0x00000004,
    /// eECollisionShapeType_Sphere    
    Sphere = 0x00000005,
    /// eECollisionShapeType_Point     
    Point = 0x00000007,
}

// eCColorSrcCombiner.CombinerType
/// eEColorSrcCombinerType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ColorSrcCombinerType {
    /// eEColorSrcCombinerType_Add     
    Add = 0x00000000,
    /// eEColorSrcCombinerType_Subtract
    Subtract = 0x00000001,
    /// eEColorSrcCombinerType_Multiply
    Multiply = 0x00000002,
    /// eEColorSrcCombinerType_Max     
    Max = 0x00000003,
    /// eEColorSrcCombinerType_Min     
    Min = 0x00000004,
}

// eCColorSrcSampler.TexRepeatU
// eCColorSrcSampler.TexRepeatV
/// eEColorSrcSampleTexRepeat
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ColorSrcSampleTexRepeat {
    /// eEColorSrcSampleTexRepeat_Wrap  
    Wrap = 0x00000000,
    /// eEColorSrcSampleTexRepeat_Clamp
    Clamp = 0x00000001,
    /// eEColorSrcSampleTexRepeat_Mirror
    Mirror = 0x00000002,
}

// eCColorSrcConstantSwitch.SwitchRepeat
// eCColorSrcSampler.SwitchRepeat
/// eEColorSrcSwitchRepeat
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ColorSrcSwitchRepeat {
    /// eEColorSrcSwitchRepeat_Repeat  
    Repeat = 0x00000000,
    /// eEColorSrcSwitchRepeat_Clamp   
    Clamp = 0x00000001,
    /// eEColorSrcSwitchRepeat_PingPong
    PingPong = 0x00000002,
}

// eCParticle_PS.CoordinateSystem
/// eECoordinateSystem
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CoordinateSystem {
    /// eECoordinateSystem_Independent
    Independent = 0x00000000,
    /// eECoordinateSystem_Relative   
    Relative = 0x00000001,
}

// eCColorSrcCamDistance.DistanceType
/// eEDistanceType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum DistanceType {
    /// eEDistanceType_Src  
    Src = 0x00000000,
    /// eEDistanceType_Dest
    Dest = 0x00000001,
    /// eEDistanceType_Delta
    Delta = 0x00000002,
}

// eCGuiWindow2.Dock
/// eEDock
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Dock {
    /// eEDock_None  
    None = 0x00000000,
    /// eEDock_Left  
    Left = 0x00000001,
    /// eEDock_Top   
    Top = 0x00000002,
    /// eEDock_Right
    Right = 0x00000003,
    /// eEDock_Bottom
    Bottom = 0x00000004,
    /// eEDock_Fill  
    Fill = 0x00000005,
}

// eCPointLight_PS.Effect
/// eEDynamicLightEffect
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum DynamicLightEffect {
    /// eEDynamicLightEffect_Steady
    Steady = 0x00000000,
    /// eEDynamicLightEffect_Flicker
    Flicker = 0x00000003,
    /// eEDynamicLightEffect_Pulse  
    Pulse = 0x00000001,
    /// eEDynamicLightEffect_Blink  
    Blink = 0x00000002,
    /// eEDynamicLightEffect_Strobe
    Strobe = 0x00000004,
}

// eCParticle_PS.FacingDirection
/// eEFacingDirection
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum FacingDirection {
    /// eEFacingDirection_FacingCamera             
    FacingCamera = 0x00000000,
    /// eEFacingDirection_AlongMovementFacingCamera
    AlongMovementFacingCamera = 0x00000001,
    /// eEFacingDirection_SpecifiedNormal          
    SpecifiedNormal = 0x00000002,
    /// eEFacingDirection_AlongMovementFacingNormal
    AlongMovementFacingNormal = 0x00000003,
    /// eEFacingDirection_PerpendicularToMovement  
    PerpendicularToMovement = 0x00000004,
}

// eCColorSrcFresnel.Term
/// eEFresnelTerm
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum FresnelTerm {
    /// eEFresnelTerm_Simple
    Simple = 0x00000000,
    /// eEFresnelTerm_Quadric
    Quadric = 0x00000001,
    /// eEFresnelTerm_Power  
    Power = 0x00000002,
}

// eCGuiCursor2.CursorSize
/// eEGuiCursorSize
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum GuiCursorSize {
    /// eEGuiCursorSize_FromSystem
    FromSystem = 0x00000000,
    /// eEGuiCursorSize_FromImage  
    FromImage = 0x00000001,
    /// eEGuiCursorSize_Independent
    Independent = 0x00000002,
}

// eCImageFilterRTBase.OutputMode
/// eEIFOutputMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum IFOutputMode {
    /// eEIFOutputMode_Texture    
    Texture = 0x00000000,
    /// eEIFOutputMode_FrameBuffer
    FrameBuffer = 0x00000001,
}

// eCImageFilterRTBase.SizeMode
/// eEIFSizeMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum IFSizeMode {
    /// eEIFSizeMode_Relative
    Relative = 0x00000001,
    /// eEIFSizeMode_Absolute
    Absolute = 0x00000000,
}

// eCImageFilterTexture.TextureMode
/// eEIFTextureMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum IFTextureMode {
    /// eEIFTextureMode_Custom         
    Custom = 0x00000000,
    /// eEIFTextureMode_SolidBackBuffer
    SolidBackBuffer = 0x00000001,
    /// eEIFTextureMode_DepthBuffer    
    DepthBuffer = 0x00000002,
}

// eCGuiImage2.BlendMode
/// eEImageBlend
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ImageBlend {
    /// eEImageBlend_AlphaBlend
    AlphaBlend = 0x00000000,
    /// eEImageBlend_Add       
    Add = 0x00001000,
    /// eEImageBlend_AddScaled
    AddScaled = 0x00002000,
    /// eEImageBlend_Modulate  
    Modulate = 0x00003000,
    /// eEImageBlend_Modulate2X
    Modulate2X = 0x00004000,
    /// eEImageBlend_Overwrite
    Overwrite = 0x00005000,
}

// eCGuiLayeredImage2.BackgroundBlendMode
// eCGuiLayeredImage2.OverlayBlendMode
/// eEImageLayerBlend
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ImageLayerBlend {
    /// eEImageLayerBlend_AlphaBlend
    AlphaBlend = 0x00010000,
    /// eEImageLayerBlend_Add       
    Add = 0x00020000,
    /// eEImageLayerBlend_AddScaled
    AddScaled = 0x00030000,
    /// eEImageLayerBlend_Modulate  
    Modulate = 0x00040000,
    /// eEImageLayerBlend_Modulate2X
    Modulate2X = 0x00050000,
    /// eEImageLayerBlend_Overwrite
    Overwrite = 0x00060000,
    /// eEImageLayerBlend_FromImage
    FromImage = 0x00000000,
}

// eCParticle_PS.LightingStyle
/// eELightingStyle
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum LightingStyle {
    /// eELightingStyle_Disabled
    Disabled = 0x00000000,
    /// eELightingStyle_Flat    
    Flat = 0x00000001,
    /// eELightingStyle_Particle
    Particle = 0x00000002,
    /// eELightingStyle_System  
    System = 0x00000003,
}

// eCGuiListCtrl2.View
/// eEListView
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ListView {
    /// eEListView_Icon     
    Icon = 0x00000000,
    /// eEListView_Details  
    Details = 0x00000001,
    /// eEListView_SmallIcon
    SmallIcon = 0x00000002,
    /// eEListView_List     
    List = 0x00000003,
    /// eEListView_Tile     
    Tile = 0x00000004,
    /// eEListView_UserGrid
    UserGrid = 0x00000005,
}

// eCGuiListCtrl2.UGIconAlignMode
// eCGuiListCtrl2.UGLabelAlignMode
// eCGuiListCtrl2.UGSubLabelAlignMode
/// eEListViewAlign
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ListViewAlign {
    /// eEListViewAlign_LeftTop     
    LeftTop = 0x00000000,
    /// eEListViewAlign_CenterTop   
    CenterTop = 0x00000001,
    /// eEListViewAlign_RightTop    
    RightTop = 0x00000002,
    /// eEListViewAlign_LeftMiddle  
    LeftMiddle = 0x00000003,
    /// eEListViewAlign_CenterMiddle
    CenterMiddle = 0x00000004,
    /// eEListViewAlign_RightMiddle
    RightMiddle = 0x00000005,
    /// eEListViewAlign_LeftBottom  
    LeftBottom = 0x00000006,
    /// eEListViewAlign_CenterBottom
    CenterBottom = 0x00000007,
    /// eEListViewAlign_RightBottom
    RightBottom = 0x00000008,
}

// eCGuiListCtrl2.UGIconSizeMode
/// eEListViewIconSize
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ListViewIconSize {
    /// eEListViewIconSize_FromImageList
    FromImageList = 0x00000000,
    /// eEListViewIconSize_FixedWidth   
    FixedWidth = 0x00000001,
    /// eEListViewIconSize_FixedHeight  
    FixedHeight = 0x00000002,
    /// eEListViewIconSize_FixedSize    
    FixedSize = 0x00000003,
}

// eCGuiListCtrl2.UGItemLayoutMode
/// eEListViewItemLayout
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ListViewItemLayout {
    /// eEListViewItemLayout_LabelRight
    LabelRight = 0x00000000,
    /// eEListViewItemLayout_LabelBottom
    LabelBottom = 0x00000001,
    /// eEListViewItemLayout_LabelLeft  
    LabelLeft = 0x00000002,
    /// eEListViewItemLayout_LabelTop   
    LabelTop = 0x00000003,
    /// eEListViewItemLayout_NoSplit    
    NoSplit = 0x00000004,
}

// eCGuiListCtrl2.TileSizeMode
// eCGuiListCtrl2.UGTileSizeMode
/// eEListViewTileSize
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ListViewTileSize {
    /// eEListViewTileSize_AutoSize   
    AutoSize = 0x00000000,
    /// eEListViewTileSize_FixedWidth
    FixedWidth = 0x00000001,
    /// eEListViewTileSize_FixedHeight
    FixedHeight = 0x00000002,
    /// eEListViewTileSize_FixedSize  
    FixedSize = 0x00000003,
}

// eCParticle_PS.StartLocationShape
/// eELocationShape
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum LocationShape {
    /// eELocationShape_Box   
    Box = 0x00000000,
    /// eELocationShape_Sphere
    Sphere = 0x00000001,
    /// eELocationShape_Mesh  
    Mesh = 0x00000002,
}

// eCParticle_PS.StartLocationTarget
/// eELocationTarget
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum LocationTarget {
    /// eELocationTarget_Self
    #[serde(rename = "Self")]
    _Self = 0x00000000,
    /// eELocationTarget_Parent
    Parent = 0x00000001,
    /// eELocationTarget_Target
    Target = 0x00000002,
}

// eCMoverAnimationBase.PlayBackMode
/// eEMoverPlayBackMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum MoverPlayBackMode {
    /// eEMoverPlayBackMode_Forward
    Forward = 0x00000000,
    /// eEMoverPlayBackMode_Backward
    Backward = 0x00000001,
    /// eEMoverPlayBackMode_PingPong
    PingPong = 0x00000002,
}

// eCGuiPictureBox2.OverlayMode
/// eEOverlayMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum OverlayMode {
    /// eEOverlayMode_Disabled  
    Disabled = 0x00000000,
    /// eEOverlayMode_Background
    Background = 0x00000001,
    /// eEOverlayMode_Picture   
    Picture = 0x00000002,
    /// eEOverlayMode_Text      
    Text = 0x00000003,
}

// eCCollisionShape_PS.Range
/// eEPhysicRangeType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum PhysicRangeType {
    /// eEPhysicRangeType_World          
    World = 0x00000000,
    /// eEPhysicRangeType_ProcessingRange
    ProcessingRange = 0x00000001,
    /// eEPhysicRangeType_VisibilityRange
    VisibilityRange = 0x00000002,

    Unknown1 = 0x1bcab0e8,
}

// eCGuiPictureBox2.PictureMode
/// eEPictureMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum PictureMode {
    /// eEPictureMode_Scale
    Scale = 0x00000000,
    /// eEPictureMode_Center
    Center = 0x00000001,
    /// eEPictureMode_Repeat
    Repeat = 0x00000002,
    /// eEPictureMode_Fit   
    Fit = 0x00000003,
}

// gCEffectCommandModifyEntity.PropertySet
/// eEPropertySetType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum PropertySetType {
    /// eEPropertySetType_Particle    
    Particle = 0x00000049,
    /// eEPropertySetType_AudioEmitter
    AudioEmitter = 0x0000004B,
}

// eCTexCoordSrcReflect.ReflectType
/// eEReflectType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ReflectType {
    /// eEReflectType_Reflect      
    Reflect = 0x00000000,
    /// eEReflectType_WorldEye     
    WorldEye = 0x00000001,
    /// eEReflectType_WorldNormal  
    WorldNormal = 0x00000002,
    /// eEReflectType_TangentNormal
    TangentNormal = 0x00000003,
    /// eEReflectType_TangentEye   
    TangentEye = 0x00000004,
}

// eCRigidBody_PS.BodyFlag
/// eERigidbody_Flag
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum RigidbodyFlag {
    /// eERigidbody_Flag_NONE           
    NONE = 0x00000000,
    /// eERigidbody_Flag_FROZEN         
    FROZEN = 0x0000007E,
    /// eERigidbody_Flag_DISABLE_GRAVITY
    GRAVITY = 0x00000001,
    /// eERigidbody_Flag_Kinematic      
    Kinematic = 0x00000080,
}

// eCParticle_PS.UseRotationFrom
/// eERotationFrom
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum RotationFrom {
    /// eERotationFrom_None  
    None = 0x00000000,
    /// eERotationFrom_Entity
    Entity = 0x00000001,
    /// eERotationFrom_Offset
    Offset = 0x00000002,
    /// eERotationFrom_Normal
    Normal = 0x00000003,
}

// eCShaderDefault.BRDFLightingType
/// eEShaderMaterialBRDFType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ShaderMaterialBRDFType {
    /// eEShaderMaterialBRDFType_Simple    
    Simple = 0x00000000,
    /// eEShaderMaterialBRDFType_Complex   
    Complex = 0x00000001,
    /// eEShaderMaterialBRDFType_WrapAround
    WrapAround = 0x00000002,
}

// eCBillboard_PS.BlendMode
// eCShaderBase.BlendMode
/// eEShaderMaterialBlendMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ShaderMaterialBlendMode {
    /// eEShaderMaterialBlendMode_Normal       
    Normal = 0x00000000,
    /// eEShaderMaterialBlendMode_Masked       
    Masked = 0x00000001,
    /// eEShaderMaterialBlendMode_AlphaBlend   
    AlphaBlend = 0x00000002,
    /// eEShaderMaterialBlendMode_Modulate     
    Modulate = 0x00000003,
    /// eEShaderMaterialBlendMode_AlphaModulate
    AlphaModulate = 0x00000004,
    /// eEShaderMaterialBlendMode_Translucent  
    Translucent = 0x00000005,
    /// eEShaderMaterialBlendMode_Brighten     
    Brighten = 0x00000007,
    /// eEShaderMaterialBlendMode_Darken       
    Darken = 0x00000006,
    /// eEShaderMaterialBlendMode_Invisible    
    Invisible = 0x00000008,
}

// eCShaderDefault.TransformationType
/// eEShaderMaterialTransformation
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ShaderMaterialTransformation {
    /// eEShaderMaterialTransformation_Default        
    Default = 0x00000000,
    /// eEShaderMaterialTransformation_Instanced      
    Instanced = 0x00000001,
    /// eEShaderMaterialTransformation_Skinned        
    Skinned = 0x00000002,
    /// eEShaderMaterialTransformation_Tree_Branches  
    Branches = 0x00000003,
    /// eEShaderMaterialTransformation_Tree_Fronds    
    Fronds = 0x00000004,
    /// eEShaderMaterialTransformation_Tree_LeafCards
    LeafCards = 0x00000005,
    /// eEShaderMaterialTransformation_Tree_LeafMeshes
    LeafMeshes = 0x00000006,
    /// eEShaderMaterialTransformation_Billboard      
    Billboard = 0x00000007,
    /// eEShaderMaterialTransformation_Morphed        
    Morphed = 0x00000008,
    /// eEShaderMaterialTransformation_Skinned_Morphed
    SkinnedMorphed = 0x00000009,
    /// eEShaderMaterialTransformation_Vegetation     
    Vegetation = 0x0000000A,
    /// eEShaderMaterialTransformation_Tree_Billboards
    Billboards = 0x0000000B,
}

// eCShaderBase.MaxShaderVersion
/// eEShaderMaterialVersion
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ShaderMaterialVersion {
    /// eEShaderMaterialVersion_1_1
    V1_1 = 0x00000000,
    /// eEShaderMaterialVersion_1_4
    V1_4 = 0x00000001,
    /// eEShaderMaterialVersion_2_0
    V2_0 = 0x00000002,
    /// eEShaderMaterialVersion_3_0
    V3_0 = 0x00000003,
}

// eCIlluminated_PS.ShadowCasterType
/// eEShadowCasterType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ShadowCasterType {
    /// eEShadowCasterType_Terrain
    Terrain = 0x00000000,
    /// eEShadowCasterType_Building
    Building = 0x00000001,
    /// eEShadowCasterType_Object  
    Object = 0x00000002,
    /// eEShadowCasterType_None    
    None = 0x00000003,
}

// eCStaticPointLight_PS.ShadowMaskIndex
/// eEShadowMaskIndex
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ShadowMaskIndex {
    /// eEShadowMaskIndex_R
    R = 0x00000002,
    /// eEShadowMaskIndex_G
    G = 0x00000001,
    /// eEShadowMaskIndex_B
    B = 0x00000000,
}

// eCCollisionShape.ShapeAABBAdaptMode
/// eEShapeAABBAdapt
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ShapeAABBAdapt {
    /// eEShapeAABBAdapt_None     
    None = 0x00000000,
    /// eEShapeAABBAdapt_LocalNode
    LocalNode = 0x00000001,
    /// eEShapeAABBAdapt_LocalTree
    LocalTree = 0x00000002,
}

// eCCollisionShape.Group
// gCStateGraphEventFilterCollisionShape.Group
/// eEShapeGroup
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ShapeGroup {
    /// eEShapeGroup_Static           
    Static = 0x00000001,
    /// eEShapeGroup_Dynamic          
    Dynamic = 0x00000002,
    /// eEShapeGroup_Shield           
    Shield = 0x00000003,
    /// eEShapeGroup_MeleeWeapon      
    MeleeWeapon = 0x00000004,
    /// eEShapeGroup_Projectile       
    Projectile = 0x00000005,
    /// eEShapeGroup_Movement         
    Movement = 0x00000006,
    /// eEShapeGroup_WeaponTrigger    
    WeaponTrigger = 0x00000007,
    /// eEShapeGroup_ParadeSphere     
    ParadeSphere = 0x00000008,
    /// eEShapeGroup_Tree_Trunk       
    Trunk = 0x00000009,
    /// eEShapeGroup_Tree_Branches    
    Branches = 0x0000000A,
    /// eEShapeGroup_Camera           
    Camera = 0x0000000B,
    /// eEShapeGroup_Movement_ZoneNPC
    ZoneNPC = 0x0000000C,
    /// eEShapeGroup_HeightRepulsor   
    HeightRepulsor = 0x0000000D,
    /// eEShapeGroup_Cloth            
    Cloth = 0x0000000E,
    /// eEShapeGroup_PhysicalBodyPart
    PhysicalBodyPart = 0x0000000F,
    /// eEShapeGroup_KeyframedBodyPart
    KeyframedBodyPart = 0x00000010,
    /// eEShapeGroup_Camera_Obstacle  
    Obstacle = 0x00000011,
    /// eEShapeGroup_Projectile_Level
    Level = 0x00000012,
    /// eEShapeGroup_Trigger          
    Trigger = 0x00000013,
    /// eEShapeGroup_Door             
    Door = 0x00000014,
}

// eCMaterialResource2.PhysicMaterial
/// eCMaterialResource2_eEShapeMaterial
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum MaterialResource2ShapeMaterial {
    /// eEShapeMaterial_None            
    None = 0x00000000,
    /// eEShapeMaterial_Wood            
    Wood = 0x00000001,
    /// eEShapeMaterial_Metal           
    Metal = 0x00000002,
    /// eEShapeMaterial_Water           
    Water = 0x00000003,
    /// eEShapeMaterial_Stone           
    Stone = 0x00000004,
    /// eEShapeMaterial_Earth           
    Earth = 0x00000005,
    /// eEShapeMaterial_Ice             
    Ice = 0x00000006,
    /// eEShapeMaterial_Leather         
    Leather = 0x00000007,
    /// eEShapeMaterial_Clay            
    Clay = 0x00000008,
    /// eEShapeMaterial_Glass           
    Glass = 0x00000009,
    /// eEShapeMaterial_Flesh           
    Flesh = 0x0000000A,
    /// eEShapeMaterial_Snow            
    Snow = 0x0000000B,
    /// eEShapeMaterial_Debris          
    Debris = 0x0000000C,
    /// eEShapeMaterial_Foliage         
    Foliage = 0x0000000D,
    /// eEShapeMaterial_Magic           
    Magic = 0x0000000E,
    /// eEShapeMaterial_Grass           
    Grass = 0x0000000F,
    /// eEShapeMaterial_SpringAndDamper1
    SpringAndDamper1 = 0x00000010,
    /// eEShapeMaterial_SpringAndDamper2
    SpringAndDamper2 = 0x00000011,
    /// eEShapeMaterial_SpringAndDamper3
    SpringAndDamper3 = 0x00000012,
}

// eCCollisionShape.Material
// gCStateGraphEventFilterCollisionShape.Material
/// eEShapeMaterial
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ShapeMaterial {
    /// eEShapeMaterial_None            
    None = 0x00000000,
    /// eEShapeMaterial_Wood            
    Wood = 0x00000001,
    /// eEShapeMaterial_Metal           
    Metal = 0x00000002,
    /// eEShapeMaterial_Water           
    Water = 0x00000003,
    /// eEShapeMaterial_Stone           
    Stone = 0x00000004,
    /// eEShapeMaterial_Earth           
    Earth = 0x00000005,
    /// eEShapeMaterial_Ice             
    Ice = 0x00000006,
    /// eEShapeMaterial_Leather         
    Leather = 0x00000007,
    /// eEShapeMaterial_Clay            
    Clay = 0x00000008,
    /// eEShapeMaterial_Glass           
    Glass = 0x00000009,
    /// eEShapeMaterial_Flesh           
    Flesh = 0x0000000A,
    /// eEShapeMaterial_Snow            
    Snow = 0x0000000B,
    /// eEShapeMaterial_Debris          
    Debris = 0x0000000C,
    /// eEShapeMaterial_Foliage         
    Foliage = 0x0000000D,
    /// eEShapeMaterial_Magic           
    Magic = 0x0000000E,
    /// eEShapeMaterial_Grass           
    Grass = 0x0000000F,
    /// eEShapeMaterial_SpringAndDamper1
    SpringAndDamper1 = 0x00000010,
    /// eEShapeMaterial_SpringAndDamper2
    SpringAndDamper2 = 0x00000011,
    /// eEShapeMaterial_SpringAndDamper3
    SpringAndDamper3 = 0x00000012,
    /// eEShapeMaterial_Damage          
    Damage = 0x00000013,
    /// eEShapeMaterial_Sand            
    Sand = 0x00000014,
    /// eEShapeMaterial_Movement        
    Movement = 0x00000015,
    /// eEShapeMaterial_Axe             
    Axe = 0x00000016,
}

// eCGuiSplitImage2.DrawStyle
/// eESplitImageStyle
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum SplitImageStyle {
    /// eESplitImageStyle_Scale         
    Scale = 0x0000001B,
    /// eESplitImageStyle_Repeat        
    Repeat = 0x00000024,
    /// eESplitImageStyle_ScaleX_RepeatY
    ScaleXRepeatY = 0x00000023,
    /// eESplitImageStyle_RepeatX_ScaleY
    RepeatXScaleY = 0x0000001C,
}

// eCIlluminated_PS.StaticIlluminated
/// eEStaticIlluminated
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum StaticIlluminated {
    /// eEStaticIlluminated_Static
    Static = 0x00000000,
    /// eEStaticIlluminated_Dynamic
    Dynamic = 0x00000001,
}

// eCStrip_PS.SpawnMode
/// eEStripSpawning
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum StripSpawning {
    /// eEStripSpawning_Movement  
    Movement = 0x00000001,
    /// eEStripSpawning_Continuous
    Continuous = 0x00000000,
    /// eEStripSpawning_Timed     
    Timed = 0x00000002,
}

// eCTexCoordSrcOscillator.OscillatorTypeU
// eCTexCoordSrcOscillator.OscillatorTypeV
/// eETexCoordSrcOscillatorType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum TexCoordSrcOscillatorType {
    /// eETexCoordSrcOscillatorType_Pan          
    Pan = 0x00000000,
    /// eETexCoordSrcOscillatorType_Stretch      
    Stretch = 0x00000001,
    /// eETexCoordSrcOscillatorType_StretchRepeat
    StretchRepeat = 0x00000002,
    /// eETexCoordSrcOscillatorType_Jitter       
    Jitter = 0x00000003,
}

// eCTexCoordSrcRotator.RotationType
/// eETexCoordSrcRotatorType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum TexCoordSrcRotatorType {
    /// eETexCoordSrcRotatorType_Once     
    Once = 0x00000000,
    /// eETexCoordSrcRotatorType_Constant
    Constant = 0x00000001,
    /// eETexCoordSrcRotatorType_Oscillate
    Oscillate = 0x00000002,
}

// eCGuiButton2.TextAlign
// eCGuiStatic2.TextAlign
/// eETextAlign
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum TextAlign {
    /// eETextAlign_Left_Top     
    LeftTop = 0x00000000,
    /// eETextAlign_Center_Top   
    CenterTop = 0x00000001,
    /// eETextAlign_Right_Top    
    RightTop = 0x00000002,
    /// eETextAlign_Left_Middle  
    LeftMiddle = 0x00000004,
    /// eETextAlign_Center_Middle
    CenterMiddle = 0x00000005,
    /// eETextAlign_Right_Middle
    RightMiddle = 0x00000006,
    /// eETextAlign_Left_Bottom  
    LeftBottom = 0x00000008,
    /// eETextAlign_Center_Bottom
    CenterBottom = 0x00000009,
    /// eETextAlign_Right_Bottom
    RightBottom = 0x0000000A,
}

// eCParticle_PS.DrawStyle
/// eETextureDrawStyle
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum TextureDrawStyle {
    /// eETextureDrawStyle_Regular      
    Regular = 0x00000000,
    /// eETextureDrawStyle_AlphaBlend   
    AlphaBlend = 0x00000001,
    /// eETextureDrawStyle_Modulated    
    Modulated = 0x00000002,
    /// eETextureDrawStyle_Translucent  
    Translucent = 0x00000003,
    /// eETextureDrawStyle_AlphaModulate
    AlphaModulate = 0x00000004,
    /// eETextureDrawStyle_Darken       
    Darken = 0x00000005,
    /// eETextureDrawStyle_Brighten     
    Brighten = 0x00000006,
    /// eETextureDrawStyle_Invisible    
    Invisible = 0x00000007,
}

// eCGuiTrackBar2.TicSide
/// eETicSide
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum TicSide {
    /// eETicSide_Right
    Right = 0x00000000,
    /// eETicSide_Left
    Left = 0x00000001,
    /// eETicSide_Both
    Both = 0x00000002,
}

// eCVegetationBrush_PS.ColorFunction
/// eEVegetationBrushColorFunction
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum VegetationBrushColorFunction {
    /// eEVegetationBrushColorFunction_Random                        
    Random = 0x00000001,
    /// eEVegetationBrushColorFunction_PerlinNoise                   
    PerlinNoise = 0x00000002,
    /// eEVegetationBrushColorFunction_PerlinNoise_Improved          
    Improved = 0x00000003,
    /// eEVegetationBrushColorFunction_EbertNoise                    
    EbertNoise = 0x00000004,
    /// eEVegetationBrushColorFunction_PeacheyNoise                  
    PeacheyNoise = 0x00000005,
    /// eEVegetationBrushColorFunction_PeacheyNoise_Gradient         
    Gradient = 0x00000006,
    /// eEVegetationBrushColorFunction_PeacheyNoise_GradientValue    
    GradientValue = 0x00000007,
    /// eEVegetationBrushColorFunction_PeacheyNoise_SparseConvolusion
    SparseConvolusion = 0x00000008,
    /// eEVegetationBrushColorFunction_PeacheyNoise_ValueConvolusion
    ValueConvolusion = 0x00000009,
}

// eCVegetationBrush_PS.Mode
/// eEVegetationBrushMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum VegetationBrushMode {
    /// eEVegetationBrushMode_Place   
    Place = 0x00000000,
    /// eEVegetationBrushMode_Remove  
    Remove = 0x00000001,
    /// eEVegetationBrushMode_Colorize
    Colorize = 0x00000002,
}

// eCVegetationBrush_PS.Placement
/// eEVegetationBrushPlace
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum VegetationBrushPlace {
    /// eEVegetationBrushPlace_DistanceSelf
    DistanceSelf = 0x00000000,
    /// eEVegetationBrushPlace_DistanceOther
    DistanceOther = 0x00000001,
    /// eEVegetationBrushPlace_RemoveOther  
    RemoveOther = 0x00000002,
}

// eCVegetationBrush_PS.ProbabilityFunction
/// eEVegetationBrushProbabilityFunction
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum VegetationBrushProbabilityFunction {
    /// eEVegetationBrushProbabilityFunction_None                          
    None = 0x00000000,
    /// eEVegetationBrushProbabilityFunction_Shape                         
    Shape = 0x00000001,
    /// eEVegetationBrushProbabilityFunction_PerlinNoise                   
    PerlinNoise = 0x00000002,
    /// eEVegetationBrushProbabilityFunction_PerlinNoise_Improved          
    Improved = 0x00000003,
    /// eEVegetationBrushProbabilityFunction_EbertNoise                    
    EbertNoise = 0x00000004,
    /// eEVegetationBrushProbabilityFunction_PeacheyNoise                  
    PeacheyNoise = 0x00000005,
    /// eEVegetationBrushProbabilityFunction_PeacheyNoise_Gradient         
    Gradient = 0x00000006,
    /// eEVegetationBrushProbabilityFunction_PeacheyNoise_GradientValue    
    GradientValue = 0x00000007,
    /// eEVegetationBrushProbabilityFunction_PeacheyNoise_SparseConvolusion
    SparseConvolusion = 0x00000008,
    /// eEVegetationBrushProbabilityFunction_PeacheyNoise_ValueConvolusion
    ValueConvolusion = 0x00000009,
}

// eCVegetationBrush_PS.Shape
/// eEVegetationBrushShape
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum VegetationBrushShape {
    /// eEVegetationBrushShape_Circle
    Circle = 0x00000000,
    /// eEVegetationBrushShape_Rect  
    Rect = 0x00000001,
    /// eEVegetationBrushShape_Single
    Single = 0x00000002,
}

// eCVegetation_Mesh.MeshShading
/// eEVegetationMeshShading
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum VegetationMeshShading {
    /// eEVegetationMeshShading_MeshNormal      
    MeshNormal = 0x00000000,
    /// eEVegetationMeshShading_EntryOrientation
    EntryOrientation = 0x00000001,
}

// eCParticle_PS.VelocityDirectionFrom
/// eEVelocityDirectionFrom
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum VelocityDirectionFrom {
    /// eEVelocityDirectionFrom_None                 
    None = 0x00000000,
    /// eEVelocityDirectionFrom_StartPositionAndOwner
    StartPositionAndOwner = 0x00000001,
    /// eEVelocityDirectionFrom_OwnerAndStartPosition
    OwnerAndStartPosition = 0x00000002,
    /// eEVelocityDirectionFrom_Owner                
    Owner = 0x00000003,
    /// eEVelocityDirectionFrom_World                
    World = 0x00000004,
}

// eCWeatherZone_PS.AmbientBackLightOverwrite
// eCWeatherZone_PS.AmbientGeneralOverwrite
// eCWeatherZone_PS.AmbientIntensityOverwrite
// eCWeatherZone_PS.CloudColorOverwrite
// eCWeatherZone_PS.CloudThicknessrOverwrite
// eCWeatherZone_PS.FogColorOverwrite
// eCWeatherZone_PS.FogDensityOverwrite
// eCWeatherZone_PS.FogEndrOverwrite
// eCWeatherZone_PS.FogStartrOverwrite
// eCWeatherZone_PS.HazeColorOverwrite
// eCWeatherZone_PS.LightDiffuseOverwrite
// eCWeatherZone_PS.LightIntensityOverwrite
// eCWeatherZone_PS.LightSpecularOverwrite
// eCWeatherZone_PS.SkyColorOverwrite
/// eEWeatherZoneOverwrite
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum WeatherZoneOverwrite {
    /// eEWeatherZoneOverwrite_None     
    None = 0x00000000,
    /// eEWeatherZoneOverwrite_Overwrite
    Overwrite = 0x00000001,
    /// eEWeatherZoneOverwrite_Modulate
    Modulate = 0x00000002,
    /// eEWeatherZoneOverwrite_Add      
    Add = 0x00000003,
}

// eCWeatherZone_PS.Shape
/// eEWeatherZoneShape
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum WeatherZoneShape {
    /// eEWeatherZoneShape_2D_Circle
    Circle = 0x00000000,
    /// eEWeatherZoneShape_2D_Rect  
    Rect = 0x00000001,
    /// eEWeatherZoneShape_3D_Sphere
    Sphere = 0x00000002,
    /// eEWeatherZoneShape_3D_Box   
    Box = 0x00000003,
}

// gCScriptRoutine_PS.AIMode
/// gEAIMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum AIMode {
    /// gEAIMode_None    
    None = 0x00000000,
    /// gEAIMode_Sender  
    Sender = 0x00000001,
    /// gEAIMode_Routine
    Routine = 0x00000002,
    /// gEAIMode_Sleep   
    Sleep = 0x00000003,
    /// gEAIMode_Observe
    Observe = 0x00000004,
    /// gEAIMode_Talk    
    Talk = 0x00000005,
    /// gEAIMode_GotoBody
    GotoBody = 0x00000006,
    /// gEAIMode_Watch   
    Watch = 0x00000007,
    /// gEAIMode_Avoid   
    Avoid = 0x00000008,
    /// gEAIMode_Threaten
    Threaten = 0x00000009,
    /// gEAIMode_Attack  
    Attack = 0x0000000A,
    /// gEAIMode_Down    
    Down = 0x0000000B,
    /// gEAIMode_Dead    
    Dead = 0x0000000C,
}

// gCAchievementBar.ViewMode
/// gEAchievementViewMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum AchievementViewMode {
    /// gEAchievementViewMode_Counter
    Counter = 0x00000000,
    /// gEAchievementViewMode_Credits
    Credits = 0x00000001,
}

// gCCombatMoveMelee.AlignToTarget
/// gEAlignToTarget
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum AlignToTarget {
    /// gEAlignToTarget_None     
    None = 0x00000000,
    /// gEAlignToTarget_TargetDir
    TargetDir = 0x00000001,
    /// gEAlignToTarget_Target   
    Target = 0x00000002,
    /// gEAlignToTarget_Free     
    Free = 0x00000003,
}

// gCScriptRoutine_PS.AmbientAction
/// gEAmbientAction
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum AmbientAction {
    /// gEAmbientAction_Ambient
    Ambient = 0x00000000,
    /// gEAmbientAction_Listen
    Listen = 0x00000001,
    /// gEAmbientAction_EMPTY  
    EMPTY = 0x00000002,
}

// gCInvAmountPicbox.Type
/// gEAmountType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum AmountType {
    /// gEAmountType_Gold    
    Gold = 0x00000000,
    /// gEAmountType_Sale    
    Sale = 0x00000001,
    /// gEAmountType_Purchase
    Purchase = 0x00000002,
}

// gCAnchor_PS.AnchorType
/// gEAnchorType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum AnchorType {
    /// gEAnchorType_Local
    Local = 0x00000000,
    /// gEAnchorType_Roam  
    Roam = 0x00000001,
    /// gEAnchorType_Patrol
    Patrol = 0x00000002,
    /// gEAnchorType_Event
    Event = 0x00000003,
}

// gCScriptRoutine_PS.AniState
// gCScriptRoutine_PS.FallbackAniState
/// gEAniState
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum AniState {
    /// gEAniState_Dummy0       
    Dummy0 = 0x00000000,
    /// gEAniState_Dummy1       
    Dummy1 = 0x00000001,
    /// gEAniState_Stand        
    Stand = 0x00000002,
    /// gEAniState_Sneak        
    Sneak = 0x00000003,
    /// gEAniState_Attack       
    Attack = 0x00000004,
    /// gEAniState_Parade       
    Parade = 0x00000005,
    /// gEAniState_Kneel        
    Kneel = 0x00000006,
    /// gEAniState_SitGround    
    SitGround = 0x00000007,
    /// gEAniState_SitStool     
    SitStool = 0x00000008,
    /// gEAniState_SitBench     
    SitBench = 0x00000009,
    /// gEAniState_SitThrone    
    SitThrone = 0x0000000A,
    /// gEAniState_SleepBed     
    SleepBed = 0x0000000B,
    /// gEAniState_SleepGround  
    SleepGround = 0x0000000C,
    /// gEAniState_SitBathtub   
    SitBathtub = 0x0000000D,
    /// gEAniState_Down         
    Down = 0x0000000E,
    /// gEAniState_DownBack     
    DownBack = 0x0000000F,
    /// gEAniState_Dead         
    Dead = 0x00000010,
    /// gEAniState_DeadBack     
    DeadBack = 0x00000011,
    /// gEAniState_Finished     
    Finished = 0x00000012,
    /// gEAniState_FinishedBack
    FinishedBack = 0x00000013,
    /// gEAniState_TalkStand    
    TalkStand = 0x00000014,
    /// gEAniState_TalkSitGround
    TalkSitGround = 0x00000015,
    /// gEAniState_TalkSitStool
    TalkSitStool = 0x00000016,
    /// gEAniState_TalkSitBench
    TalkSitBench = 0x00000017,
    /// gEAniState_TalkSitThrone
    TalkSitThrone = 0x00000018,
    /// gEAniState_Wade         
    Wade = 0x00000019,
    /// gEAniState_Swim         
    Swim = 0x0000001A,
    /// gEAniState_Dive         
    Dive = 0x0000001B,
    /// gEAniState_Stumble      
    Stumble = 0x0000001C,
    /// gEAniState_Levitate     
    Levitate = 0x0000001D,
}

// gCCombatMoveStumble.ResultingAniState
/// gCCombatMoveStumble_gEAniState
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CombatMoveStumbleAniState {
    /// gEAniState_Stand   
    Stand = 0x00000002,
    /// gEAniState_Down    
    Down = 0x0000000E,
    /// gEAniState_DownBack
    DownBack = 0x0000000F,
    /// gEAniState_Dead    
    Dead = 0x00000010,
    /// gEAniState_DeadBack
    DeadBack = 0x00000011,
}

// gCArena_PS.Status
/// gEArenaStatus
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ArenaStatus {
    /// gEArenaStatus_None   
    None = 0x00000000,
    /// gEArenaStatus_Running
    Running = 0x00000001,
}

// gCNPC_PS.AttitudeLock
/// gEAttitude
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Attitude {
    /// gEAttitude_None    
    None = 0x00000000,
    /// gEAttitude_Friendly
    Friendly = 0x00000001,
    /// gEAttitude_Neutral
    Neutral = 0x00000002,
    /// gEAttitude_Angry   
    Angry = 0x00000003,
    /// gEAttitude_Hostile
    Hostile = 0x00000004,
}

// gCInfoCommandBoostAttribs.BoostTarget
/// gEBoostTarget
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum BoostTarget {
    /// gEBoostTarget_Strength    
    Strength = 0x00000000,
    /// gEBoostTarget_Dexterity   
    Dexterity = 0x00000001,
    /// gEBoostTarget_Intelligence
    Intelligence = 0x00000002,
}

// gCNPC_PS.BraveryOverride
/// gEBraveryOverride
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum BraveryOverride {
    /// gEBraveryOverride_None  
    None = 0x00000000,
    /// gEBraveryOverride_Brave
    Brave = 0x00000001,
    /// gEBraveryOverride_Coward
    Coward = 0x00000002,
}

// gCCombatMoveMelee.Action
/// gCCombatMoveMelee_gECombatAction
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum MeleeCombatAction {
    /// gECombatAction_Attack
    Attack = 0x00000001,
    /// gECombatAction_Parade
    Parade = 0x00000002,
    /// gECombatAction_Stumble
    Stumble = 0x00000003,
}

// gCCombatMoveScriptState.MoveCombatAction
/// gCCombatMoveScriptState_gECombatAction
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum MoveCombatAction {
    /// gECombatAction_None   
    None = 0x00000000,
    /// gECombatAction_Attack
    Attack = 0x00000001,
    /// gECombatAction_Parade
    Parade = 0x00000002,
    /// gECombatAction_Stumble
    Stumble = 0x00000003,
}

// gCCombatMoveMelee.AttackStumbleType
/// gECombatAttackStumble
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CombatAttackStumble {
    /// gECombatAttackStumble_None      
    None = 0x00000000,
    /// gECombatAttackStumble_ParadeType
    ParadeType = 0x00000001,
}

// gCCombatMoveMelee.ComboParade
/// gECombatComboParade
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CombatComboParade {
    /// gECombatComboParade_None   
    None = 0x00000000,
    /// gECombatComboParade_Parade
    Parade = 0x00000001,
    /// gECombatComboParade_Parade1
    Parade1 = 0x00000002,
}

// gCCombatSystem_PS.FightAIMode
/// gECombatFightAIMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CombatFightAIMode {
    /// gECombatFightAIMode_Active
    Active = 0x00000000,
    /// gECombatFightAIMode_Passive
    Passive = 0x00000001,
}

// gCCombatMoveMelee.HitDirection
/// gECombatHitDirection
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CombatHitDirection {
    /// gECombatHitDirection_Fore
    Fore = 0x00000000,
    /// gECombatHitDirection_Left
    Left = 0x00000001,
    /// gECombatHitDirection_Right
    Right = 0x00000002,
}

// gCCombatStyle.CombatMode
/// gECombatMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CombatMode {
    /// gECombatMode_None  
    None = 0x00000000,
    /// gECombatMode_Carry
    Carry = 0x00000001,
    /// gECombatMode_Melee
    Melee = 0x00000002,
    /// gECombatMode_Ranged
    Ranged = 0x00000003,
    /// gECombatMode_Magic
    Magic = 0x00000004,
    /// gECombatMode_Cast  
    Cast = 0x00000005,
}

// gCCombatMoveMelee.ParadeReaction
/// gCCombatMoveMelee_gECombatMove
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum MeleeCombatMove {
    /// gECombatMove_None              
    None = 0x00000000,
    /// gECombatMove_ParadeStumble     
    ParadeStumble = 0x0000000E,
    /// gECombatMove_ParadeStumbleHeavy
    ParadeStumbleHeavy = 0x0000000F,
    /// gECombatMove_AttackStumble     
    AttackStumble = 0x0000000B,
    /// gECombatMove_AttackStumbleLeft
    AttackStumbleLeft = 0x0000000C,
    /// gECombatMove_AttackStumbleRight
    AttackStumbleRight = 0x0000000D,
}

// gCCombatMoveMelee.StumbleReaction
/// gCCombatMoveMelee2_gECombatMove
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Melee2CombatMove {
    /// gECombatMove_Stumble           
    Stumble = 0x00000019,
    /// gECombatMove_StumbleLight      
    StumbleLight = 0x0000001A,
    /// gECombatMove_StumbleHeavy      
    StumbleHeavy = 0x0000001B,
    /// gECombatMove_StumbleBack       
    StumbleBack = 0x0000001F,
    /// gECombatMove_StumbleDown       
    StumbleDown = 0x00000020,
    /// gECombatMove_ParadeStumble     
    ParadeStumble = 0x0000000E,
    /// gECombatMove_ParadeStumbleHeavy
    ParadeStumbleHeavy = 0x0000000F,
}

// gCCombatMoveMelee.WeaponSide
// gCCombatMoveParade.WeaponSide
/// gECombatMoveSide
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CombatMoveSide {
    /// gECombatMoveSide_Left
    Left = 0x00000000,
    /// gECombatMoveSide_Right
    Right = 0x00000001,
}

// gCCombatMoveMelee.AttackType
// gCCombatStyle.AttackType
// gCCombatStyle.ParadeType
/// gECombatParadeType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CombatParadeType {
    /// gECombatParadeType_None   
    None = 0x00000000,
    /// gECombatParadeType_Fist   
    Fist = 0x00000001,
    /// gECombatParadeType_Weapon
    Weapon = 0x00000002,
    /// gECombatParadeType_Magic  
    Magic = 0x00000004,
    /// gECombatParadeType_Ranged
    Ranged = 0x00000008,
    /// gECombatParadeType_Monster
    Monster = 0x00000010,
    /// gECombatParadeType_Shield
    Shield = 0x0000001B,
}

// gCCombatMoveMeleePhase.PhaseType
/// gECombatPhaseType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CombatPhaseType {
    /// gECombatPhaseType_Raise        
    Raise = 0x00000001,
    /// gECombatPhaseType_PowerRaise   
    PowerRaise = 0x00000002,
    /// gECombatPhaseType_Hit          
    Hit = 0x00000003,
    /// gECombatPhaseType_Recover      
    Recover = 0x00000005,
    /// gECombatPhaseType_Hoover       
    Hoover = 0x00000004,
    /// gECombatPhaseType_Parade       
    Parade = 0x00000006,
    /// gECombatPhaseType_Strafe       
    Strafe = 0x00000007,
    /// gECombatPhaseType_CounterParade
    CounterParade = 0x00000008,
    /// gECombatPhaseType_CounterAttack
    CounterAttack = 0x00000009,
}

// gCCombatStyleAniPose.Pose
/// gECombatPose
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CombatPose {
    /// gECombatPose_P0
    P0 = 0x00000000,
    /// gECombatPose_P1
    P1 = 0x00000001,
    /// gECombatPose_P2
    P2 = 0x00000002,
}

// gCNPC_PS.LastPlayerComment
/// gEComment
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Comment {
    /// gEComment_None             
    None = 0x00000000,
    /// gEComment_DefeatInquisition
    DefeatInquisition = 0x00000001,
    /// gEComment_Theft            
    Theft = 0x00000002,
    /// gEComment_Livestock        
    Livestock = 0x00000003,
    /// gEComment_Defeat           
    Defeat = 0x00000004,
    /// gEComment_Count            
    Count = 0x00000005,
}

// gCInfoConditionSkillValue.CompareOperation
/// gECompareOperation
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum CompareOperation {
    /// gECompareOperation_Equal       
    Equal = 0x00000000,
    /// gECompareOperation_NotEqual    
    NotEqual = 0x00000001,
    /// gECompareOperation_Less        
    Less = 0x00000002,
    /// gECompareOperation_LessEqual   
    LessEqual = 0x00000003,
    /// gECompareOperation_Greater     
    Greater = 0x00000004,
    /// gECompareOperation_GreaterEqual
    GreaterEqual = 0x00000005,
}

// gCNPC_PS.LastPlayerCrime
/// gECrime
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Crime {
    /// gECrime_None           
    None = 0x00000000,
    /// gECrime_MurderLivestock
    MurderLivestock = 0x00000001,
    /// gECrime_Theft          
    Theft = 0x00000002,
    /// gECrime_Murder         
    Murder = 0x00000003,
    /// gECrime_Count          
    Count = 0x00000004,
}

// gCNPC_PS.DamageCalculationType
/// gEDamageCalculationType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum DamageCalculationType {
    /// gEDamageCalculationType_Normal  
    Normal = 0x00000000,
    /// gEDamageCalculationType_Monster
    Monster = 0x00000001,
    /// gEDamageCalculationType_Immortal
    Immortal = 0x00000002,
}

// gCDamage_PS.DamageType
/// gEDamageType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum DamageType {
    /// gEDamageType_None   
    None = 0x00000000,
    /// gEDamageType_Edge   
    Edge = 0x00000001,
    /// gEDamageType_Blunt  
    Blunt = 0x00000002,
    /// gEDamageType_Point  
    Point = 0x00000003,
    /// gEDamageType_Fire   
    Fire = 0x00000004,
    /// gEDamageType_Ice    
    Ice = 0x00000005,
    /// gEDamageType_Magic  
    Magic = 0x00000006,
    /// gEDamageType_Physics
    Physics = 0x00000007,
}

// gCDoor_PS.Status
/// gEDoorStatus
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum DoorStatus {
    /// gEDoorStatus_Open  
    Open = 0x00000000,
    /// gEDoorStatus_Closed
    Closed = 0x00000001,
}

// gCEffect_PS.DecayMode
/// gEEffectDecayMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum EffectDecayMode {
    /// gEEffectDecayMode_Decay
    Decay = 0x00000000,
    /// gEEffectDecayMode_Kill
    Kill = 0x00000001,
}

// gCEffectCommandKillEntityRange.Range
/// gEEffectKillRange
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum EffectKillRange {
    /// gEEffectKillRange_All  
    All = 0x00000000,
    /// gEEffectKillRange_Range
    Range = 0x00000001,
}

// gCEffectCommandPlaySound.CoordinateSystem
// gCEffectCommandSpawnEntity.CoordinateSystem
// gCEffectCommandSpawnEntityList.CoordinateSystem
// gCEffectCommandSpawnEntitySwitch.CoordinateSystem
/// gEEffectLink
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum EffectLink {
    /// gEEffectLink_Independent
    Independent = 0x00000000,
    /// gEEffectLink_TargetEntity
    TargetEntity = 0x00000001,
    /// gEEffectLink_TargetBone  
    TargetBone = 0x00000002,
}

// gCEffect_PS.LoopMode
/// gEEffectLoopMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum EffectLoopMode {
    /// gEEffectLoopMode_Once  
    Once = 0x00000000,
    /// gEEffectLoopMode_Loop  
    Loop = 0x00000001,
    /// gEEffectLoopMode_Repeat
    Repeat = 0x00000002,
}

// gCEffectCommandRunScript.OtherType
/// gEEffectScriptOtherType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum EffectScriptOtherType {
    /// gEEffectScriptOtherType_TemplateEntity
    TemplateEntity = 0x00000001,
    /// gEEffectScriptOtherType_Entity        
    Entity = 0x00000000,
}

// gCEffectCommandRunScript.ParamType
/// gEEffectScriptParamType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum EffectScriptParamType {
    /// gEEffectScriptParamType_UseEffectCommandTime
    UseEffectCommandTime = 0x00000001,
    /// gEEffectScriptParamType_UseParam            
    UseParam = 0x00000000,
}

// gCEffect_PS.StopMode
/// gEEffectStopMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum EffectStopMode {
    /// gEEffectStopMode_Decay  
    Decay = 0x00000000,
    /// gEEffectStopMode_Disable
    Disable = 0x00000001,
    /// gEEffectStopMode_Kill   
    Kill = 0x00000002,
}

// gCEffect_PS.TargetMode
/// gEEffectTargetMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum EffectTargetMode {
    /// gEEffectTargetMode_Self  
    #[serde(rename = "Self")]
    _Self = 0x00000000,
    /// gEEffectTargetMode_Parent
    Parent = 0x00000001,
    /// gEEffectTargetMode_Script
    Script = 0x00000002,
}

// gCDynamicLayer.EntityType
/// gEEntityType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum EntityType {
    /// gEEntityType_Game     
    Game = 0x00000000,
    /// gEEntityType_Temporary
    Temporary = 0x00000001,
}

// gCEquipPicbox2.EquipSlot
/// gCEquipPicbox2_gEEquipSlot
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Picbox2EquipSlot {
    /// gEEquipSlot_MeleeWeapon
    MeleeWeapon = 0x00000001,
    /// gEEquipSlot_MeleeShield
    MeleeShield = 0x00000002,
    /// gEEquipSlot_RangedWeapon
    RangedWeapon = 0x00000003,
    /// gEEquipSlot_RangedAmmo  
    RangedAmmo = 0x00000004,
    /// gEEquipSlot_Amulet      
    Amulet = 0x00000005,
    /// gEEquipSlot_Ring1       
    Ring1 = 0x00000006,
    /// gEEquipSlot_Ring2       
    Ring2 = 0x00000007,
    /// gEEquipSlot_Armor       
    Armor = 0x00000008,
    /// gEEquipSlot_Helmet      
    Helmet = 0x00000009,
}

// gCInventoryStack.EquipSlot
/// gEEquipSlot
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum EquipSlot {
    /// gEEquipSlot_None        
    None = 0x00000000,
    /// gEEquipSlot_MeleeWeapon
    MeleeWeapon = 0x00000001,
    /// gEEquipSlot_MeleeShield
    MeleeShield = 0x00000002,
    /// gEEquipSlot_RangedWeapon
    RangedWeapon = 0x00000003,
    /// gEEquipSlot_RangedAmmo  
    RangedAmmo = 0x00000004,
    /// gEEquipSlot_Amulet      
    Amulet = 0x00000005,
    /// gEEquipSlot_Ring1       
    Ring1 = 0x00000006,
    /// gEEquipSlot_Ring2       
    Ring2 = 0x00000007,
    /// gEEquipSlot_Armor       
    Armor = 0x00000008,
    /// gEEquipSlot_Helmet      
    Helmet = 0x00000009,
}

// gCNPC_PS.LastFightAgainstPlayer
/// gEFight
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Fight {
    /// gEFight_None   
    None = 0x00000000,
    /// gEFight_Lost   
    Lost = 0x00000001,
    /// gEFight_Won    
    Won = 0x00000002,
    /// gEFight_Cancel
    Cancel = 0x00000003,
    /// gEFight_Running
    Running = 0x00000004,
}

// gCProjectile2_PS.FlightPathType
/// gEFlightPathType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum FlightPathType {
    /// gEFlightPathType_Ballistic
    Ballistic = 0x00000000,
    /// gEFlightPathType_Seeking  
    Seeking = 0x00000001,
}

// gCInteraction_PS.FocusNameType
/// gEFocusNameType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum FocusNameType {
    /// gEFocusNameType_Skeleton
    Skeleton = 0x00000000,
    /// gEFocusNameType_Entity  
    Entity = 0x00000001,
    /// gEFocusNameType_Bone    
    Bone = 0x00000002,
    /// gEFocusNameType_Disable
    Disable = 0x00000003,
    /// gEFocusNameType_Center  
    Center = 0x00000004,
}

// gCInteraction_PS.FocusPriority
/// gEFocusPriority
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum FocusPriority {
    /// gEFocusPriority_None   
    None = 0x00000000,
    /// gEFocusPriority_Lowest
    Lowest = 0x00000001,
    /// gEFocusPriority_Low    
    Low = 0x00000002,
    /// gEFocusPriority_Normal
    Normal = 0x00000003,
    /// gEFocusPriority_High   
    High = 0x00000004,
    /// gEFocusPriority_Highest
    Highest = 0x00000005,
}

// gCFocusInteractFilter2.Source
/// gEFocusSource
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum FocusSource {
    /// gEFocusSource_Camera            
    Camera = 0x00000000,
    /// gEFocusSource_Player            
    Player = 0x00000001,
    /// gEFocusSource_PlayerPosCameraDir
    PlayerPosCameraDir = 0x00000002,
    /// gEFocusSource_CameraPosPlayerDir
    CameraPosPlayerDir = 0x00000003,
    /// gEFocusSource_Auto              
    Auto = 0x00000004,
}

// gCGUIFilter.FilterType
/// gEGUIFilterType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum GUIFilterType {
    /// gEGUIFilterType_Status     
    Status = 0x00000001,
    /// gEGUIFilterType_NPCInfoType
    NPCInfoType = 0x00000002,
    /// gEGUIFilterType_Location   
    Location = 0x00000003,
    /// gEGUIFilterType_Category   
    Category = 0x00000004,
    /// gEGUIFilterType_InfoType   
    InfoType = 0x00000005,
    /// gEGUIFilterType_Details    
    Details = 0x00000006,
}

// gCGammaScrollBar2.GammaRamp
// gCGammaTrackbar2.GammaRamp
/// gEGammaRamp
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum GammaRamp {
    /// gEGammaRamp_Brightness
    Brightness = 0x00000000,
    /// gEGammaRamp_Contrast  
    Contrast = 0x00000001,
    /// gEGammaRamp_Red       
    Red = 0x00000002,
    /// gEGammaRamp_Green     
    Green = 0x00000003,
    /// gEGammaRamp_Blue      
    Blue = 0x00000004,
}

// gCNPC_PS.Gender
/// gEGender
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Gender {
    /// gEGender_Male  
    Male = 0x00000000,
    /// gEGender_Female
    Female = 0x00000001,
}

// gCInfoCommandSetGuardStatus.GuardStatus
/// gEGuardStatus
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum GuardStatus {
    /// gEGuardStatus_Active         
    Active = 0x00000000,
    /// gEGuardStatus_FirstWarnGiven
    FirstWarnGiven = 0x00000001,
    /// gEGuardStatus_SecondWarnGiven
    SecondWarnGiven = 0x00000002,
    /// gEGuardStatus_Inactive       
    Inactive = 0x00000003,
}

// gCNPC_PS.GuardStatus
/// gCNPC_PS_gEGuardStatus
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum NpcGuardStatus {
    /// gEGuardStatus_Active         
    Active = 0x00000000,
    /// gEGuardStatus_FirstWarnGiven
    FirstWarnGiven = 0x00000001,
    /// gEGuardStatus_SecondWarnGiven
    SecondWarnGiven = 0x00000002,
    /// gEGuardStatus_Inactive       
    Inactive = 0x00000003,
    /// gEGuardStatus_Behind         
    Behind = 0x00000004,
}

// gCAIZone_PS.Guild
/// gEGuild
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Guild {
    /// gEGuild_None
    None = 0x00000000,
    /// gEGuild_Don
    Don = 0x00000001,
    /// gEGuild_Dig
    Dig = 0x00000002,
    /// gEGuild_Grd
    Grd = 0x00000003,
    /// gEGuild_Cit
    Cit = 0x00000004,
    /// gEGuild_Inq
    Inq = 0x00000005,
    /// gEGuild_Mag
    Mag = 0x00000006,
    /// gEGuild_Pir
    Pir = 0x00000007,
}

// gCNPC_PS.Guild
/// gCNPC_PS_gEGuild
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum NpcGuild {
    /// gEGuild_None
    None = 0x00000000,
    /// gEGuild_Don  
    Don = 0x00000001,
    /// gEGuild_Dig  
    Dig = 0x00000002,
    /// gEGuild_Grd  
    Grd = 0x00000003,
    /// gEGuild_Cit  
    Cit = 0x00000004,
    /// gEGuild_Inq  
    Inq = 0x00000005,
    /// gEGuild_Mag  
    Mag = 0x00000006,
    /// gEGuild_Pir  
    Pir = 0x00000007,
    /// gEGuild_Count
    Count = 0x00000008,
}

// gCScriptRoutine_PS.HitDirection
/// gEHitDirection
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum HitDirection {
    /// gEHitDirection_Left
    Left = 0x00000000,
    /// gEHitDirection_Right
    Right = 0x00000001,
}

// gCHudPage2.PageID
/// gEHudPage
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum HudPage {
    /// gEHudPage_None            
    None = 0x00000000,
    /// gEHudPage_Game            
    Game = 0x00000001,
    /// gEHudPage_Inventory       
    Inventory = 0x00000002,
    /// gEHudPage_Character       
    Character = 0x00000003,
    /// gEHudPage_Log             
    Log = 0x00000004,
    /// gEHudPage_Map             
    Map = 0x00000005,
    /// gEHudPage_CraftSelect     
    CraftSelect = 0x00000006,
    /// gEHudPage_ItemSelect      
    ItemSelect = 0x00000007,
    /// gEHudPage_Loot            
    Loot = 0x00000008,
    /// gEHudPage_Pickpocket      
    Pickpocket = 0x00000009,
    /// gEHudPage_Trade           
    Trade = 0x0000000A,
    /// gEHudPage_Dialog          
    Dialog = 0x0000000B,
    /// gEHudPage_Talk            
    Talk = 0x0000000C,
    /// gEHudPage_Menu_Back       
    MenuBack = 0x0000001A,
    /// gEHudPage_Menu_Main       
    MenuMain = 0x0000000D,
    /// gEHudPage_Menu_Game       
    MenuGame = 0x0000000E,
    /// gEHudPage_Menu_Load       
    MenuLoad = 0x0000000F,
    /// gEHudPage_Menu_Save       
    MenuSave = 0x00000010,
    /// gEHudPage_Menu_Achievement
    MenuAchievement = 0x00000011,
    /// gEHudPage_Menu_Options    
    MenuOptions = 0x00000012,
    /// gEHudPage_Menu_Video      
    MenuVideo = 0x00000013,
    /// gEHudPage_Menu_Audio      
    MenuAudio = 0x00000014,
    /// gEHudPage_Menu_Input      
    MenuInput = 0x00000015,
    /// gEHudPage_Menu_Settings   
    MenuSettings = 0x00000016,
    /// gEHudPage_Menu_System     
    MenuSystem = 0x00000017,
    /// gEHudPage_Menu_Credits    
    MenuCredits = 0x00000018,
    /// gEHudPage_Menu_Cheats     
    MenuCheats = 0x00000019,
    /// gEHudPage_Outro           
    Outro = 0x0000001B,
    /// gEHudPage_Loading         
    Loading = 0x0000001C,
}

// gCPageTimerProgressBar.HudPage
/// gCPageTimerProgressBar_gEHudPage
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum HudPageProgessBar {
    /// gEHudPage_Pickpocket
    Pickpocket = 0x00000009,
    /// gEHudPage_CraftSelect
    CraftSelect = 0x00000006,
    /// gEHudPage_ItemSelect
    ItemSelect = 0x00000007,
    /// gEHudPage_Loot       
    Loot = 0x00000008,
}

// gCInventoryList.Icon
/// gEIcon
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Icon {
    /// gEIcon_Inventory
    Inventory = 0x00000000,
    /// gEIcon_Craft    
    Craft = 0x00000001,
}

// gCInfo.ConditionType
/// gEInfoCondType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum InfoCondType {
    /// gEInfoCondType_Crime         
    Crime = 0x00000000,
    /// gEInfoCondType_Duel          
    Duel = 0x00000001,
    /// gEInfoCondType_Hello         
    Hello = 0x00000002,
    /// gEInfoCondType_General       
    General = 0x00000003,
    /// gEInfoCondType_Overtime      
    Overtime = 0x00000004,
    /// gEInfoCondType_Open          
    Open = 0x00000005,
    /// gEInfoCondType_Activator     
    Activator = 0x00000006,
    /// gEInfoCondType_Running       
    Running = 0x00000007,
    /// gEInfoCondType_Delivery      
    Delivery = 0x00000008,
    /// gEInfoCondType_PartDelivery  
    PartDelivery = 0x00000009,
    /// gEInfoCondType_Success       
    Success = 0x0000000A,
    /// gEInfoCondType_DoCancel      
    DoCancel = 0x0000000B,
    /// gEInfoCondType_Failed        
    Failed = 0x0000000C,
    /// gEInfoCondType_Cancelled     
    Cancelled = 0x0000000D,
    /// gEInfoCondType_Join          
    Join = 0x0000000E,
    /// gEInfoCondType_Dismiss       
    Dismiss = 0x0000000F,
    /// gEInfoCondType_Trade         
    Trade = 0x00000011,
    /// gEInfoCondType_PickPocket    
    PickPocket = 0x00000012,
    /// gEInfoCondType_Ready         
    Ready = 0x00000013,
    /// gEInfoCondType_Lost          
    Lost = 0x00000014,
    /// gEInfoCondType_Reactivator   
    Reactivator = 0x00000015,
    /// gEInfoCondType_Won           
    Won = 0x00000016,
    /// gEInfoCondType_MasterThief   
    MasterThief = 0x00000017,
    /// gEInfoCondType_FirstWarn     
    FirstWarn = 0x0000001A,
    /// gEInfoCondType_SecondWarn    
    SecondWarn = 0x0000001B,
    /// gEInfoCondType_MobJoin       
    MobJoin = 0x0000001C,
    /// gEInfoCondType_SlaveJoin     
    SlaveJoin = 0x0000001D,
    /// gEInfoCondType_LongTimeNoSee
    LongTimeNoSee = 0x0000001E,
    /// gEInfoCondType_PoliticalCrime
    PoliticalCrime = 0x00000020,
    /// gEInfoCondType_MobDismiss    
    MobDismiss = 0x00000021,
    /// gEInfoCondType_Wait          
    Wait = 0x00000022,
    /// gEInfoCondType_Heal          
    Heal = 0x00000023,
    /// gEInfoCondType_NothingToSay  
    NothingToSay = 0x00000032,
    /// gEInfoCondType_End           
    End = 0x00000033,
    /// gEInfoCondType_Back          
    Back = 0x00000034,
    /// gEInfoCondType_Finished      
    Finished = 0x00000036,
    /// gEInfoCondType_NotYetFinished
    NotYetFinished = 0x00000038,
}

// gCInfoConditionQuestStatus.CondType
/// gCInfoConditionQuestStatus_gEInfoCondType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum QuestInfoCondType {
    /// gEInfoCondType_Open          
    Open = 0x00000005,
    /// gEInfoCondType_Overtime      
    Overtime = 0x00000004,
    /// gEInfoCondType_Running       
    Running = 0x00000007,
    /// gEInfoCondType_Success       
    Success = 0x0000000A,
    /// gEInfoCondType_Failed        
    Failed = 0x0000000C,
    /// gEInfoCondType_Cancelled     
    Cancelled = 0x0000000D,
    /// gEInfoCondType_Ready         
    Ready = 0x00000013,
    /// gEInfoCondType_Lost          
    Lost = 0x00000014,
    /// gEInfoCondType_Won           
    Won = 0x00000016,
    /// gEInfoCondType_NotYetFinished
    NotYetFinished = 0x00000035,
    /// gEInfoCondType_Finished      
    Finished = 0x00000036,
}

// gCInfoCommandPickPocket.Gesture
// gCInfoCommandSay.Gesture
// gCInfoCommandSaySVM.Gesture
// gCInfoCommandThink.Gesture
/// gEInfoGesture
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum InfoGesture {
    /// gEInfoGesture_Ambient
    Ambient = 0x00000000,
    /// gEInfoGesture_Me      
    Me = 0x00000001,
    /// gEInfoGesture_You     
    You = 0x00000002,
    /// gEInfoGesture_Threaten
    Threaten = 0x00000003,
    /// gEInfoGesture_Yes     
    Yes = 0x00000004,
    /// gEInfoGesture_No      
    No = 0x00000005,
    /// gEInfoGesture_All     
    All = 0x00000006,
    /// gEInfoGesture_Back    
    Back = 0x00000007,
    /// gEInfoGesture_Tell    
    Tell = 0x00000008,
    /// gEInfoGesture_Admonish
    Admonish = 0x00000009,
    /// gEInfoGesture_Secret  
    Secret = 0x0000000A,
    /// gEInfoGesture_Recall  
    Recall = 0x0000000B,
    /// gEInfoGesture_YouAndMe
    YouAndMe = 0x0000000C,
}

// gCInfoCommandAddNPCInfo.Location
// gCInfoCommandRemoveNPCInfo.Location
// gCQuest.LocationInfo
/// gEInfoLocation
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum InfoLocation {
    /// gEInfoLocation_Main     
    Main = 0x00000000,
    /// gEInfoLocation_Harbor   
    Harbor = 0x00000001,
    /// gEInfoLocation_Monastery
    Monastery = 0x00000002,
    /// gEInfoLocation_Don      
    Don = 0x00000003,
}

// gCInfoConditionNPCStatus.SecondaryNPCStatus
/// gEInfoNPCStatus
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum InfoNPCStatus {
    /// gEInfoNPCStatus_Alive            
    Alive = 0x00000000,
    /// gEInfoNPCStatus_UnHarmed         
    UnHarmed = 0x00000001,
    /// gEInfoNPCStatus_Defeated         
    Defeated = 0x00000002,
    /// gEInfoNPCStatus_Dead             
    Dead = 0x00000003,
    /// gEInfoNPCStatus_TalkedToPlayer   
    TalkedToPlayer = 0x00000004,
    /// gEInfoNPCStatus_NotTalkedToPlayer
    NotTalkedToPlayer = 0x00000005,
}

// gCInfoCommandAddNPCInfo.Type
// gCInfoCommandRemoveNPCInfo.Type
/// gEInfoNPCType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum InfoNPCType {
    /// gEInfoNPCType_Vendor
    Vendor = 0x00000000,
    /// gEInfoNPCType_Teacher
    Teacher = 0x00000001,
    /// gEInfoNPCType_VIP    
    VIP = 0x00000002,
}

// gCInfo.Type
/// gEInfoType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum InfoType {
    /// gEInfoType_Refuse   
    Refuse = 0x00000000,
    /// gEInfoType_Important
    Important = 0x00000001,
    /// gEInfoType_News     
    News = 0x00000002,
    /// gEInfoType_Info     
    Info = 0x00000003,
    /// gEInfoType_Parent   
    Parent = 0x00000004,
    /// gEInfoType_Comment  
    Comment = 0x00000005,
}

// gCItemInfo.ItemDetails
/// gEInfoView
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum InfoView {
    /// gEInfoView_Header             
    Header = 0x00000000,
    /// gEInfoView_Description        
    Description = 0x00000001,
    /// gEInfoView_Damage             
    Damage = 0x00000002,
    /// gEInfoView_RequiredSkills     
    RequiredSkills = 0x00000003,
    /// gEInfoView_ModifySkills       
    ModifySkills = 0x00000004,
    /// gEInfoView_RequiredIngredients
    RequiredIngredients = 0x00000005,
    /// gEInfoView_GoldValue          
    GoldValue = 0x00000006,
    /// gEInfoView_CraftRequiredSkills
    CraftRequiredSkills = 0x00000007,
    /// gEInfoView_RequiredMana       
    RequiredMana = 0x00000008,
}

// gCSkillInfo.ItemDetails
/// gCSkillInfo_gEInfoView
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum SkillInfoView {
    /// gEInfoView_Header     
    Header = 0x00000000,
    /// gEInfoView_Description
    Description = 0x00000001,
    /// gEInfoView_Level      
    Level = 0x00000002,
    /// gEInfoView_NextLevel  
    NextLevel = 0x00000003,
}

// gCDialogInfo.View
/// gCDialogInfo_gCDialogInfo_gEInfoView
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum DialogInfoView {
    /// gEInfoView_Header     
    Header = 0x00000000,
    /// gEInfoView_Description
    Description = 0x00000001,
    /// gEInfoView_NextLevel  
    NextLevel = 0x00000002,
    /// gEInfoView_Learnpoints
    Learnpoints = 0x00000003,
    /// gEInfoView_GoldCosts  
    GoldCosts = 0x00000004,
}

// gCHintStatic.View
/// gCHintStatic_gEInfoView
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum HintInfoView {
    /// gEInfoView_Image    
    Image = 0x00000000,
    /// gEInfoView_Text     
    Text = 0x00000001,
    /// gEInfoView_ImageText
    ImageText = 0x00000002,
}

// gCInteraction.Type
/// gEInteractionType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum InteractionType {
    /// gEInteractionType_Interact_NPC       
    InteractNPC = 0x00000000,
    /// gEInteractionType_Interact_Player    
    InteractPlayer = 0x00000001,
    /// gEInteractionType_InventoryUse_Player
    InventoryUse = 0x00000002,
    /// gEInteractionType_QuickUse_Player    
    QuickUse = 0x00000003,
    /// gEInteractionType_Magic              
    Magic = 0x00000004,
}

// gCFocusInteractFilter2.UseType
/// SPECIAL_gEInteractionUseType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum SpecialInteractionUseType {
    /// gEInteractionUseType_None
    None = 0x00000000,
    /// gEInteractionUseType_Item
    Item = 0x00000001,
    /// gEInteractionUseType_NPC
    NPC = 0x00000002,
}

// gCInteraction_PS.UseType
/// gEInteractionUseType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum InteractionUseType {
    /// gEInteractionUseType_None          
    None = 0x00000000,
    /// gEInteractionUseType_Item          
    Item = 0x00000001,
    /// gEInteractionUseType_NPC           
    NPC = 0x00000002,
    /// gEInteractionUseType_Roam          
    Roam = 0x00000003,
    /// gEInteractionUseType_CoolWeapon    
    CoolWeapon = 0x00000004,
    /// gEInteractionUseType_Anvil         
    Anvil = 0x00000005,
    /// gEInteractionUseType_Forge         
    Forge = 0x00000006,
    /// gEInteractionUseType_GrindStone    
    GrindStone = 0x00000007,
    /// gEInteractionUseType_Cauldron      
    Cauldron = 0x00000008,
    /// gEInteractionUseType_Barbecue      
    Barbecue = 0x00000009,
    /// gEInteractionUseType_Alchemy       
    Alchemy = 0x0000000A,
    /// gEInteractionUseType_LookAt        
    LookAt = 0x0000000B,
    /// gEInteractionUseType_Bookstand     
    Bookstand = 0x0000000C,
    /// gEInteractionUseType_TakeCarryable
    TakeCarryable = 0x0000000D,
    /// gEInteractionUseType_DropCarryable
    DropCarryable = 0x0000000E,
    /// gEInteractionUseType_PickOre       
    PickOre = 0x0000000F,
    /// gEInteractionUseType_PickGround    
    PickGround = 0x00000010,
    /// gEInteractionUseType_DigGround     
    DigGround = 0x00000011,
    /// gEInteractionUseType_Fieldwork     
    Fieldwork = 0x00000012,
    /// gEInteractionUseType_WriteScroll   
    WriteScroll = 0x00000013,
    /// gEInteractionUseType_SawLog        
    SawLog = 0x00000014,
    /// gEInteractionUseType_PracticeStaff
    PracticeStaff = 0x00000015,
    /// gEInteractionUseType_Bed           
    Bed = 0x00000016,
    /// gEInteractionUseType_SleepGround   
    SleepGround = 0x00000017,
    /// gEInteractionUseType_SweepFloor    
    SweepFloor = 0x00000018,
    /// gEInteractionUseType_Dance         
    Dance = 0x00000019,
    /// gEInteractionUseType_Flute         
    Flute = 0x0000001A,
    /// gEInteractionUseType_Boss          
    Boss = 0x0000001B,
    /// gEInteractionUseType_Throne        
    Throne = 0x0000001C,
    /// gEInteractionUseType_Pace          
    Pace = 0x0000001D,
    /// gEInteractionUseType_Bard          
    Bard = 0x0000001E,
    /// gEInteractionUseType_Stool         
    Stool = 0x0000001F,
    /// gEInteractionUseType_Bench         
    Bench = 0x00000020,
    /// gEInteractionUseType_Waterpipe     
    Waterpipe = 0x00000021,
    /// gEInteractionUseType_Fountain      
    Fountain = 0x00000022,
    /// gEInteractionUseType_PirateTreasure
    PirateTreasure = 0x00000023,
    /// gEInteractionUseType_Campfire      
    Campfire = 0x00000024,
    /// gEInteractionUseType_Stove         
    Stove = 0x00000025,
    /// gEInteractionUseType_SitGround     
    SitGround = 0x00000026,
    /// gEInteractionUseType_Smalltalk     
    Smalltalk = 0x00000027,
    /// gEInteractionUseType_Preach        
    Preach = 0x00000028,
    /// gEInteractionUseType_Spectator     
    Spectator = 0x00000029,
    /// gEInteractionUseType_Stand         
    Stand = 0x0000002A,
    /// gEInteractionUseType_Guard         
    Guard = 0x0000002B,
    /// gEInteractionUseType_Trader        
    Trader = 0x0000002C,
    /// gEInteractionUseType_Listener      
    Listener = 0x0000002D,
    /// gEInteractionUseType_Cupboard      
    Cupboard = 0x0000002E,
    /// gEInteractionUseType_Pee           
    Pee = 0x0000002F,
    /// gEInteractionUseType_Bathtub       
    Bathtub = 0x00000030,
    /// gEInteractionUseType_Door          
    Door = 0x00000031,
    /// gEInteractionUseType_Chest         
    Chest = 0x00000032,
    /// gEInteractionUseType_Flee          
    Flee = 0x00000033,
    /// gEInteractionUseType_Lever         
    Lever = 0x00000034,
    /// gEInteractionUseType_Button        
    Button = 0x00000035,
    /// gEInteractionUseType_Winch         
    Winch = 0x00000036,
    /// gEInteractionUseType_Destructable  
    Destructable = 0x00000037,
    /// gEInteractionUseType_Goldsmith     
    Goldsmith = 0x00000038,
    /// gEInteractionUseType_Altar         
    Altar = 0x00000039,
    /// gEInteractionUseType_Sarcophagus   
    Sarcophagus = 0x0000003A,
    /// gEInteractionUseType_SecretRing    
    SecretRing = 0x0000003B,
    /// gEInteractionUseType_MagicOrb      
    MagicOrb = 0x0000003C,
    /// gEInteractionUseType_RedBarrier    
    RedBarrier = 0x0000003D,
    /// gEInteractionUseType_BlueBarrier   
    BlueBarrier = 0x0000003E,
    /// gEInteractionUseType_LizardBook    
    LizardBook = 0x0000003F,
    /// gEInteractionUseType_PracticeSword
    PracticeSword = 0x00000040,
    /// gEInteractionUseType_PracticeMagic
    PracticeMagic = 0x00000041,
    /// gEInteractionUseType_Plunder       
    Plunder = 0x00000042,
    /// gEInteractionUseType_Loo           
    Loo = 0x00000043,
    /// gEInteractionUseType_Attack        
    Attack = 0x00000044,
    /// gEInteractionUseType_Keyhole       
    Keyhole = 0x00000045,
}

// gCItem_PS.Category
/// gEItemCategory
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ItemCategory {
    /// gEItemCategory_None      
    None = 0x00000000,
    /// gEItemCategory_Weapon    
    Weapon = 0x00000001,
    /// gEItemCategory_Armor     
    Armor = 0x00000002,
    /// gEItemCategory_Consumable
    Consumable = 0x00000003,
    /// gEItemCategory_Empty_D   
    D = 0x00000004,
    /// gEItemCategory_Magic     
    Magic = 0x00000005,
    /// gEItemCategory_Misc      
    Misc = 0x00000006,
    /// gEItemCategory_Written   
    Written = 0x00000007,
    /// gEItemCategory_Empty_B   
    B = 0x00000008,
    /// gEItemCategory_Empty_E   
    E = 0x00000009,
    /// gEItemCategory_Empty_F   
    F = 0x0000000A,
    /// gEItemCategory_Count     
    Count = 0x0000000B,
}

// gCItem_PS.HoldType
/// gEItemHoldType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ItemHoldType {
    /// gEItemHoldType_None         
    None = 0x00000000,
    /// gEItemHoldType_1H           
    OneH = 0x00000001,
    /// gEItemHoldType_2H           
    TwoH = 0x00000002,
    /// gEItemHoldType_BS           
    BS = 0x00000003,
    /// gEItemHoldType_Arrow        
    Arrow = 0x00000004,
    /// gEItemHoldType_Bow          
    Bow = 0x00000005,
    /// gEItemHoldType_CrossBow     
    CrossBow = 0x00000006,
    /// gEItemHoldType_Bolt         
    Bolt = 0x00000007,
    /// gEItemHoldType_Fist         
    Fist = 0x00000008,
    /// gEItemHoldType_Shield       
    Shield = 0x00000009,
    /// gEItemHoldType_Armor        
    Armor = 0x0000000A,
    /// gEItemHoldType_Helmet       
    Helmet = 0x0000000B,
    /// gEItemHoldType_Staff        
    Staff = 0x0000000C,
    /// gEItemHoldType_Amulet       
    Amulet = 0x0000000D,
    /// gEItemHoldType_Ring         
    Ring = 0x0000000E,
    /// gEItemHoldType_Rune         
    Rune = 0x0000000F,
    /// gEItemHoldType_Torch        
    Torch = 0x00000010,
    /// gEItemHoldType_CarryFront   
    CarryFront = 0x00000011,
    /// gEItemHoldType_Axe          
    Axe = 0x00000012,
    /// gEItemHoldType_Cast         
    Cast = 0x0000001D,
    /// gEItemHoldType_FocusCast    
    FocusCast = 0x0000001F,
    /// gEItemHoldType_Magic        
    Magic = 0x0000001E,
    /// gEItemHoldType_Apple        
    Apple = 0x00000013,
    /// gEItemHoldType_Bread        
    Bread = 0x00000014,
    /// gEItemHoldType_Jar          
    Jar = 0x00000015,
    /// gEItemHoldType_Joint        
    Joint = 0x00000016,
    /// gEItemHoldType_Meat         
    Meat = 0x00000017,
    /// gEItemHoldType_Potion       
    Potion = 0x00000018,
    /// gEItemHoldType_Saringda     
    Saringda = 0x00000019,
    /// gEItemHoldType_Saw          
    Saw = 0x0000001A,
    /// gEItemHoldType_Scoop        
    Scoop = 0x0000001B,
    /// gEItemHoldType_Stew         
    Stew = 0x0000001C,
    /// gEItemHoldType_MagicMissile
    MagicMissile = 0x00000020,
    /// gEItemHoldType_MagicFireball
    MagicFireball = 0x00000021,
    /// gEItemHoldType_MagicIcelance
    MagicIcelance = 0x00000022,
    /// gEItemHoldType_Flute        
    Flute = 0x00000023,
}

// gCItem_PS.UseType
/// gEItemUseType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ItemUseType {
    /// gEItemUseType_None         
    None = 0x00000000,
    /// gEItemUseType_1H           
    OneH = 0x00000001,
    /// gEItemUseType_2H           
    TwoH = 0x00000002,
    /// gEItemUseType_BS           
    BS = 0x00000003,
    /// gEItemUseType_Arrow        
    Arrow = 0x00000004,
    /// gEItemUseType_Bow          
    Bow = 0x00000005,
    /// gEItemUseType_CrossBow     
    CrossBow = 0x00000006,
    /// gEItemUseType_Bolt         
    Bolt = 0x00000007,
    /// gEItemUseType_Fist         
    Fist = 0x00000008,
    /// gEItemUseType_Shield       
    Shield = 0x00000009,
    /// gEItemUseType_Armor        
    Armor = 0x0000000A,
    /// gEItemUseType_Helmet       
    Helmet = 0x0000000B,
    /// gEItemUseType_Staff        
    Staff = 0x0000000C,
    /// gEItemUseType_Amulet       
    Amulet = 0x0000000D,
    /// gEItemUseType_Ring         
    Ring = 0x0000000E,
    /// gEItemUseType_Rune         
    Rune = 0x0000000F,
    /// gEItemUseType_Torch        
    Torch = 0x00000010,
    /// gEItemUseType_CarryFront   
    CarryFront = 0x00000011,
    /// gEItemUseType_Axe          
    Axe = 0x00000012,
    /// gEItemUseType_Cast         
    Cast = 0x00000013,
    /// gEItemUseType_FocusCast    
    FocusCast = 0x00000014,
    /// gEItemUseType_MagicMissile
    MagicMissile = 0x00000015,
    /// gEItemUseType_MagicFireball
    MagicFireball = 0x00000016,
    /// gEItemUseType_MagicIcelance
    MagicIcelance = 0x00000017,
    /// gEItemUseType_MagicAmmo    
    MagicAmmo = 0x00000018,
    /// gEItemUseType_MagicFrost   
    MagicFrost = 0x00000019,
    /// gEItemUseType_Head         
    Head = 0x0000001A,
}

// gCCombatWeaponConfig.LeftUseType
// gCCombatWeaponConfig.RightUseType
/// gCCombatWeaponConfig_gEItemUseType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum WeaponItemUseType {
    /// gEItemUseType_None         
    None = 0x00000000,
    /// gEItemUseType_1H           
    OneH = 0x00000001,
    /// gEItemUseType_2H           
    TwoH = 0x00000002,
    /// gEItemUseType_BS           
    BS = 0x00000003,
    /// gEItemUseType_Arrow        
    Arrow = 0x00000004,
    /// gEItemUseType_Bow          
    Bow = 0x00000005,
    /// gEItemUseType_CrossBow     
    CrossBow = 0x00000006,
    /// gEItemUseType_Bolt         
    Bolt = 0x00000007,
    /// gEItemUseType_Fist         
    Fist = 0x00000008,
    /// gEItemUseType_Shield       
    Shield = 0x00000009,
    /// gEItemUseType_Staff        
    Staff = 0x0000000C,
    /// gEItemUseType_Axe          
    Axe = 0x00000012,
    /// gEItemUseType_Torch        
    Torch = 0x00000010,
    /// gEItemUseType_CarryFront   
    CarryFront = 0x00000011,
    /// gEItemUseType_Cast         
    Cast = 0x00000013,
    /// gEItemUseType_FocusCast    
    FocusCast = 0x00000014,
    /// gEItemUseType_MagicMissile
    MagicMissile = 0x00000015,
    /// gEItemUseType_MagicFireball
    MagicFireball = 0x00000016,
    /// gEItemUseType_MagicIcelance
    MagicIcelance = 0x00000017,
    /// gEItemUseType_MagicAmmo    
    MagicAmmo = 0x00000018,
    /// gEItemUseType_MagicFrost   
    MagicFrost = 0x00000019,
}

// gCLock_PS.Status
/// gELockStatus
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum LockStatus {
    /// gELockStatus_Locked  
    Locked = 0x00000000,
    /// gELockStatus_Unlocked
    Unlocked = 0x00000001,
}

// gCMiscLabel.InfoType
/// gEMiscInfo
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum MiscInfo {
    /// gEMiscInfo_Guild
    Guild = 0x00000000,
}

// gCMiscProgressBar.InfoType
/// gCMiscProgressBar_gEMiscInfo
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ProgressBarMiscInfo {
    /// gEMiscInfo_StatusEffect
    StatusEffect = 0x00000001,
    /// gEMiscInfo_Health      
    Health = 0x00000002,
    /// gEMiscInfo_Mana        
    Mana = 0x00000003,
}

// gCMouseInvAxisTrackbar2.MouseAxis
// gCMouseSensitivityTrackbar2.MouseAxis
/// gEMouseAxis
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum MouseAxis {
    /// gEMouseAxis_X
    X = 0x00000000,
    /// gEMouseAxis_Y
    Y = 0x00000001,
}

// gCCollisionCircle_PS.Type
/// gENavObstacleType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum NavObstacleType {
    /// gENavObstacleType_Obstacle
    Obstacle = 0x00000000,
    /// gENavObstacleType_Climbable
    Climbable = 0x00000001,
}

// gCAIHelper_FreePoint_PS.NavTestResult
// gCNavOffset_PS.NavTestResult
// gCNavPath_PS.NavTestResult
// gCNavZone_PS.NavTestResult
// gCNegZone_PS.NavTestResult
// gCPrefPath_PS.NavTestResult
/// gENavTestResult
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum NavTestResult {
    /// gENavTestResult_Succeeded                       
    Succeeded = 0x00000000,
    /// gENavTestResult_NavPathWithOneDeadEnd           
    NavPathWithOneDeadEnd = 0x00000001,
    /// gENavTestResult_NavPathWithTwoDeadEnds          
    NavPathWithTwoDeadEnds = 0x00000002,
    /// gENavTestResult_NavPathBlockedByCollisionCircle
    NavPathBlockedByCollisionCircle = 0x00000003,
    /// gENavTestResult_NavPathIllegalBuild             
    NavPathIllegalBuild = 0x00000004,
    /// gENavTestResult_PrefPathOutOfNavZone            
    PrefPathOutOfNavZone = 0x00000005,
    /// gENavTestResult_PrefPathBlockedByCollisionCircle
    PrefPathBlockedByCollisionCircle = 0x00000006,
    /// gENavTestResult_PrefPathIllegalBuild            
    PrefPathIllegalBuild = 0x00000007,
    /// gENavTestResult_NavZoneInConflict               
    NavZoneInConflict = 0x00000008,
    /// gENavTestResult_NavZoneIllegalBuild             
    NavZoneIllegalBuild = 0x00000009,
    /// gENavTestResult_NegZoneOutOfNavZone             
    NegZoneOutOfNavZone = 0x0000000A,
    /// gENavTestResult_NegZoneIllegalBuild             
    NegZoneIllegalBuild = 0x0000000B,
    /// gENavTestResult_FreePointOutOfNavArea           
    FreePointOutOfNavArea = 0x0000000C,
    /// gENavTestResult_FreePointInNegZone              
    FreePointInNegZone = 0x0000000D,
    /// gENavTestResult_FreePointInCollisionCircle      
    FreePointInCollisionCircle = 0x0000000E,
    /// gENavTestResult_NavOffsetOutOfNavArea           
    NavOffsetOutOfNavArea = 0x00000010,
}

// gCInfoCommandRunScript.OtherType
/// gEOtherType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum OtherType {
    /// gEOtherType_TemplateEntity
    TemplateEntity = 0x00000001,
    /// gEOtherType_Entity        
    Entity = 0x00000000,
}

// gCLootStatic2.VisibleMode
/// gCLootStatic2_gEPageMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum LootPageMode {
    /// gEPageMode_Dialog   
    Dialog = 0x00000003,
    /// gEPageMode_UserMin  
    UserMin = 0x00000004,
    /// gEPageMode_UserSlots
    UserSlots = 0x00000006,
    /// gEPageMode_UserMax  
    UserMax = 0x00000007,
}

// gCHudPage2.PageMode
/// gCHudPage2_gEPageMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum HudPageMode {
    /// gEPageMode_None     
    None = 0x00000000,
    /// gEPageMode_Panorama
    Panorama = 0x00000001,
    /// gEPageMode_Dialog   
    Dialog = 0x00000003,
    /// gEPageMode_UserMin  
    UserMin = 0x00000004,
    /// gEPageMode_UserSlots
    UserSlots = 0x00000006,
    /// gEPageMode_UserMax  
    UserMax = 0x00000007,
}

// gCCompassPicbox2.VisibleMode
// gCEquipPicbox2.VisibleMode
// gCLockpickStatic2.VisibleMode
// gCMiscProgressBar.VisibleMode
// gCQuickPicbox2.VisibleMode
// gCTutorialLabel2.VisibleMode
/// gEPageMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum PageMode {
    /// gEPageMode_UserMin  
    UserMin = 0x00000004,
    /// gEPageMode_UserSlots
    UserSlots = 0x00000006,
    /// gEPageMode_UserMax  
    UserMax = 0x00000007,
}

// gCHintStatic.PaintArea
/// gEPaintArea
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum PaintArea {
    /// gEPaintArea_Client
    Client = 0x00000000,
    /// gEPaintArea_Window
    Window = 0x00000001,
    /// gEPaintArea_Desktop
    Desktop = 0x00000002,
}

// gCParty_PS.PartyMemberType
/// gEPartyMemberType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum PartyMemberType {
    /// gEPartyMemberType_None       
    None = 0x00000000,
    /// gEPartyMemberType_Party      
    Party = 0x00000001,
    /// gEPartyMemberType_Mob        
    Mob = 0x00000002,
    /// gEPartyMemberType_Slave      
    Slave = 0x00000003,
    /// gEPartyMemberType_Controlled
    Controlled = 0x00000004,
    /// gEPartyMemberType_Summoned   
    Summoned = 0x00000005,
    /// gEPartyMemberType_PlayerGuide
    PlayerGuide = 0x00000006,
}

// gCQuestActor.ActorType
/// gEQuestActor
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum QuestActor {
    /// gEQuestActor_Client
    Client = 0x00000000,
    /// gEQuestActor_Target
    Target = 0x00000001,
}

// gCQuest.Status
/// gEQuestStatus
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum QuestStatus {
    /// gEQuestStatus_Open     
    Open = 0x00000000,
    /// gEQuestStatus_Running  
    Running = 0x00000001,
    /// gEQuestStatus_Success  
    Success = 0x00000002,
    /// gEQuestStatus_Failed   
    Failed = 0x00000003,
    /// gEQuestStatus_Obsolete
    Obsolete = 0x00000004,
    /// gEQuestStatus_Cancelled
    Cancelled = 0x00000005,
    /// gEQuestStatus_Lost     
    Lost = 0x00000006,
    /// gEQuestStatus_Won      
    Won = 0x00000007,
}

// gCQuest.Type
/// gEQuestType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum QuestType {
    /// gEQuestType_HasItems
    HasItems = 0x00000000,
    /// gEQuestType_Report   
    Report = 0x00000001,
    /// gEQuestType_Kill     
    Kill = 0x00000002,
    /// gEQuestType_Defeat   
    Defeat = 0x00000003,
    /// gEQuestType_DriveAway
    DriveAway = 0x00000004,
    /// gEQuestType_Arena    
    Arena = 0x00000005,
    /// gEQuestType_BringNpc
    BringNpc = 0x00000006,
    /// gEQuestType_FollowNpc
    FollowNpc = 0x00000007,
    /// gEQuestType_EnterArea
    EnterArea = 0x00000008,
    /// gEQuestType_Plunder  
    Plunder = 0x0000000B,
    /// gEQuestType_Sparring
    Sparring = 0x0000000C,
    /// gEQuestType_Duel     
    Duel = 0x0000000D,
}

// gCQuickPicbox2.QuickSlot
/// gEQuickSlot
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum QuickSlot {
    /// gEQuickSlot_1
    #[serde(rename = "1")]
    _1 = 0x00000000,
    /// gEQuickSlot_2
    #[serde(rename = "2")]
    _2 = 0x00000001,
    /// gEQuickSlot_3
    #[serde(rename = "3")]
    _3 = 0x00000002,
    /// gEQuickSlot_4
    #[serde(rename = "4")]
    _4 = 0x00000003,
    /// gEQuickSlot_5
    #[serde(rename = "5")]
    _5 = 0x00000004,
    /// gEQuickSlot_6
    #[serde(rename = "6")]
    _6 = 0x00000005,
    /// gEQuickSlot_7
    #[serde(rename = "7")]
    _7 = 0x00000006,
    /// gEQuickSlot_8
    #[serde(rename = "8")]
    _8 = 0x00000007,
    /// gEQuickSlot_9
    #[serde(rename = "9")]
    _9 = 0x00000008,
    /// gEQuickSlot_10
    #[serde(rename = "10")]
    _10 = 0x00000009,
}

// gCNPC_PS.LastPlayerAR
// gCNPC_PS.Reason
/// gEReason
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Reason {
    /// gEReason_None           
    None = 0x00000000,
    /// gEReason_SVM_Ambient    
    Ambient = 0x00000001,
    /// gEReason_SVM_Combat     
    Combat = 0x00000002,
    /// gEReason_SVM_Party      
    Party = 0x00000003,
    /// gEReason_PlayerTalk     
    PlayerTalk = 0x00000004,
    /// gEReason_ImportantInfo  
    ImportantInfo = 0x00000005,
    /// gEReason_PlayerSneaking
    PlayerSneaking = 0x00000006,
    /// gEReason_PlayerWeapon   
    PlayerWeapon = 0x00000007,
    /// gEReason_PlayerRoom     
    PlayerRoom = 0x00000008,
    /// gEReason_PlayerUseBed   
    PlayerUseBed = 0x00000009,
    /// gEReason_Eat            
    Eat = 0x0000000A,
    /// gEReason_Ransack        
    Ransack = 0x0000000B,
    /// gEReason_Fighter        
    Fighter = 0x0000000C,
    /// gEReason_Attacker       
    Attacker = 0x0000000D,
    /// gEReason_Nuisance       
    Nuisance = 0x0000000E,
    /// gEReason_Joke           
    Joke = 0x0000000F,
    /// gEReason_Frost          
    Frost = 0x00000010,
    /// gEReason_Damage         
    Damage = 0x00000011,
    /// gEReason_DamageLivestock
    DamageLivestock = 0x00000012,
    /// gEReason_MurderLivestock
    MurderLivestock = 0x00000013,
    /// gEReason_Theft          
    Theft = 0x00000014,
    /// gEReason_Illusion       
    Illusion = 0x00000015,
    /// gEReason_GateGuard      
    GateGuard = 0x00000016,
    /// gEReason_Defeat         
    Defeat = 0x00000017,
    /// gEReason_Inspect        
    Inspect = 0x00000018,
    /// gEReason_Finish         
    Finish = 0x00000019,
    /// gEReason_Raider         
    Raider = 0x0000001A,
    /// gEReason_Enemy          
    Enemy = 0x0000001B,
    /// gEReason_Murder         
    Murder = 0x0000001C,
    /// gEReason_Duel           
    Duel = 0x0000001D,
    /// gEReason_Arena          
    Arena = 0x0000001E,
    /// gEReason_Kill           
    Kill = 0x0000001F,
    /// gEReason_Count          
    Count = 0x00000020,
}

// gCRecipe_PS.Craft
/// gERecipeCategory
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum RecipeCategory {
    /// gERecipeCategory_Alchemy         
    Alchemy = 0x00000000,
    /// gERecipeCategory_Cooking         
    Cooking = 0x00000001,
    /// gERecipeCategory_Frying          
    Frying = 0x00000003,
    /// gERecipeCategory_Goldsmith       
    Goldsmith = 0x00000004,
    /// gERecipeCategory_WriteScroll     
    WriteScroll = 0x00000005,
    /// gERecipeCategory_Smith_Forge     
    Forge = 0x00000006,
    /// gERecipeCategory_Smith_Anvil     
    Anvil = 0x00000007,
    /// gERecipeCategory_Smith_CoolWeapon
    CoolWeapon = 0x00000008,
    /// gERecipeCategory_Smith_GrindStone
    GrindStone = 0x00000009,
}

// gCCreditsLabel2.ScrollStart
/// gEScrollStart
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ScrollStart {
    /// gEScrollStart_Top   
    Top = 0x00000000,
    /// gEScrollStart_Bottom
    Bottom = 0x00000001,
}

// gCSession.State
/// gESession_State
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum SessionState {
    /// gESession_State_None         
    None = 0x00000000,
    /// gESession_State_Movement     
    Movement = 0x00000001,
    /// gESession_State_Fight        
    Fight = 0x00000002,
    /// gESession_State_Ride_Movement
    RideMovement = 0x00000003,
    /// gESession_State_Ride_Fight   
    RideFight = 0x00000004,
    /// gESession_State_ItemUse      
    ItemUse = 0x00000005,
    /// gESession_State_Inventory    
    Inventory = 0x00000006,
    /// gESession_State_Dialog       
    Dialog = 0x00000007,
    /// gESession_State_Trade        
    Trade = 0x00000008,
    /// gESession_State_InteractObj  
    InteractObj = 0x00000009,
    /// gESession_State_Journal      
    Journal = 0x0000000A,
    /// gESession_State_Editor       
    Editor = 0x0000000B,
}

// gCSkillPicbox.SkillType
/// gESkill
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Skill {
    /// gESkill_None           
    None = 0xFFFFFFFF,
    /// gESkill_Atrib_HP       
    AttribHP = 0x00000000,
    /// gESkill_Atrib_MP       
    AttribMP = 0x00000001,
    /// gESkill_Stat_LV        
    StatLV = 0x00000002,
    /// gESkill_Stat_XP        
    StatXP = 0x00000003,
    /// gESkill_Stat_LP        
    StatLP = 0x00000004,
    /// gESkill_Stat_HP        
    StatHP = 0x00000005,
    /// gESkill_Stat_MP        
    StatMP = 0x00000006,
    /// gESkill_Stat_STR       
    StatSTR = 0x00000007,
    /// gESkill_Stat_DEX       
    StatDEX = 0x00000008,
    /// gESkill_Stat_INT       
    StatINT = 0x00000009,
    /// gESkill_Prot_Edge      
    ProtEdge = 0x0000000A,
    /// gESkill_Prot_Blunt     
    ProtBlunt = 0x0000000B,
    /// gESkill_Prot_Point     
    ProtPoint = 0x0000000C,
    /// gESkill_Prot_Fire      
    ProtFire = 0x0000000D,
    /// gESkill_Prot_Ice       
    Ice = 0x0000000E,
    /// gESkill_Prot_Magic     
    ProtMagic = 0x0000000F,
    /// gESkill_Combat_Sword   
    CombatSword = 0x00000010,
    /// gESkill_Combat_Axe     
    CombatAxe = 0x00000011,
    /// gESkill_Combat_Staff   
    CombatStaff = 0x00000012,
    /// gESkill_Combat_Bow     
    CombatBow = 0x00000013,
    /// gESkill_Combat_CrossBow
    CombatCrossBow = 0x00000014,
    /// gESkill_Magic_Circle   
    MagicCircle = 0x00000015,
    /// gESkill_Magic_Fireball
    MagicFireball = 0x00000016,
    /// gESkill_Magic_Frost    
    MagicFrost = 0x00000017,
    /// gESkill_Magic_Missile  
    MagicMissile = 0x00000018,
    /// gESkill_Misc_Scribe    
    MiscScribe = 0x00000020,
    /// gESkill_Misc_Alchemy   
    MiscAlchemy = 0x0000001F,
    /// gESkill_Misc_Smith     
    MiscSmith = 0x00000019,
    /// gESkill_Misc_Mining    
    MiscMining = 0x0000001A,
    /// gESkill_Misc_Sneak     
    MiscSneak = 0x0000001D,
    /// gESkill_Misc_Lockpick  
    MiscLockpick = 0x0000001B,
    /// gESkill_Misc_Pickpocket
    MiscPickpocket = 0x0000001C,
    /// gESkill_Misc_Acrobat   
    MiscAcrobat = 0x0000001E,
    /// gESkill_Misc_Trophy    
    MiscTrophy = 0x00000021,
}

// gCSkillValueBase.Skill
/// gCSkillValueBase_gESkill
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum SkillValueBaseSkill {
    /// gESkill_None           
    None = 0xFFFFFFFF,
    /// gESkill_Atrib_HP       
    AtribHP = 0x00000000,
    /// gESkill_Atrib_MP       
    AtribMP = 0x00000001,
    /// gESkill_Stat_LV        
    StatLV = 0x00000002,
    /// gESkill_Stat_XP        
    StatXP = 0x00000003,
    /// gESkill_Stat_LP        
    StatLP = 0x00000004,
    /// gESkill_Stat_HP        
    StatHP = 0x00000005,
    /// gESkill_Stat_MP        
    StatMP = 0x00000006,
    /// gESkill_Stat_STR       
    StatSTR = 0x00000007,
    /// gESkill_Stat_DEX       
    StatDEX = 0x00000008,
    /// gESkill_Stat_INT       
    StatINT = 0x00000009,
    /// gESkill_Prot_Edge      
    ProtEdge = 0x0000000A,
    /// gESkill_Prot_Blunt     
    ProtBlunt = 0x0000000B,
    /// gESkill_Prot_Point     
    ProtPoint = 0x0000000C,
    /// gESkill_Prot_Fire      
    ProtFire = 0x0000000D,
    /// gESkill_Prot_Ice       
    ProtIce = 0x0000000E,
    /// gESkill_Prot_Magic     
    ProtMagic = 0x0000000F,
    /// gESkill_Combat_Sword   
    CombatSword = 0x00000010,
    /// gESkill_Combat_Axe     
    CombatAxe = 0x00000011,
    /// gESkill_Combat_Staff   
    CombatStaff = 0x00000012,
    /// gESkill_Combat_Bow     
    CombatBow = 0x00000013,
    /// gESkill_Combat_CrossBow
    CombatCrossBow = 0x00000014,
    /// gESkill_Magic_Circle   
    MagicCircle = 0x00000015,
    /// gESkill_Magic_Fireball
    MagicFireball = 0x00000016,
    /// gESkill_Magic_Frost    
    MagicFrost = 0x00000017,
    /// gESkill_Magic_Missile  
    MagicMissile = 0x00000018,
    /// gESkill_Misc_Smith     
    MiscSmith = 0x00000019,
    /// gESkill_Misc_Mining    
    MiscMining = 0x0000001A,
    /// gESkill_Misc_Lockpick  
    MiscLockpick = 0x0000001B,
    /// gESkill_Misc_Pickpocket
    MiscPickpocket = 0x0000001C,
    /// gESkill_Misc_Sneak     
    MiscSneak = 0x0000001D,
    /// gESkill_Misc_Acrobat   
    MiscAcrobat = 0x0000001E,
    /// gESkill_Misc_Alchemy   
    MiscAlchemy = 0x0000001F,
    /// gESkill_Misc_Scribe    
    MiscScribe = 0x00000020,
    /// gESkill_Misc_Trophy    
    MiscTrophy = 0x00000021,
}

// gCSkillProgressBar.SkillType
/// gCSkillProgressBar_gESkill
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ProgressBarSkill {
    /// gESkill_Stat_LV        
    LV = 0x00000002,
    /// gESkill_Stat_XP        
    XP = 0x00000003,
    /// gESkill_Stat_LP        
    LP = 0x00000004,
    /// gESkill_Stat_HP        
    HP = 0x00000005,
    /// gESkill_Stat_MP        
    MP = 0x00000006,
    /// gESkill_Stat_STR       
    STR = 0x00000007,
    /// gESkill_Stat_DEX       
    DEX = 0x00000008,
    /// gESkill_Stat_INT       
    INT = 0x00000009,
    /// gESkill_Prot_Edge      
    Edge = 0x0000000A,
    /// gESkill_Prot_Blunt     
    Blunt = 0x0000000B,
    /// gESkill_Prot_Point     
    Point = 0x0000000C,
    /// gESkill_Prot_Fire      
    Fire = 0x0000000D,
    /// gESkill_Prot_Ice       
    Ice = 0x0000000E,
    /// gESkill_Prot_Magic     
    Magic = 0x0000000F,
    /// gESkill_Combat_Sword   
    Sword = 0x00000010,
    /// gESkill_Combat_Axe     
    Axe = 0x00000011,
    /// gESkill_Combat_Staff   
    Staff = 0x00000012,
    /// gESkill_Combat_Bow     
    Bow = 0x00000013,
    /// gESkill_Combat_CrossBow
    CrossBow = 0x00000014,
    /// gESkill_Magic_Circle   
    Circle = 0x00000015,
    /// gESkill_Magic_Fireball
    Fireball = 0x00000016,
    /// gESkill_Magic_Frost    
    Frost = 0x00000017,
    /// gESkill_Magic_Missile  
    Missile = 0x00000018,
    /// gESkill_Misc_Scribe    
    Scribe = 0x00000020,
    /// gESkill_Misc_Alchemy   
    Alchemy = 0x0000001F,
    /// gESkill_Misc_Smith     
    Smith = 0x00000019,
    /// gESkill_Misc_Mining    
    Mining = 0x0000001A,
    /// gESkill_Misc_Sneak     
    Sneak = 0x0000001D,
    /// gESkill_Misc_Lockpick  
    Lockpick = 0x0000001B,
    /// gESkill_Misc_Pickpocket
    Pickpocket = 0x0000001C,
    /// gESkill_Misc_Acrobat   
    Acrobat = 0x0000001E,
    /// gESkill_Misc_Trophy    
    Trophy = 0x00000021,
}

// gCModifySkill.Modifier
/// gESkillModifier
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum SkillModifier {
    /// gESkillModifier_AddValue       
    AddValue = 0x00000000,
    /// gESkillModifier_SetToMax       
    SetToMax = 0x00000001,
    /// gESkillModifier_SetToValue     
    SetToValue = 0x00000002,
    /// gESkillModifier_AddPercentOfMax
    AddPercentOfMax = 0x00000003,
}

// gCInventoryList.Character
// gCMiscProgressBar.Character
/// gESpecialEntity
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum SpecialEntity {
    /// gESpecialEntity_Player   
    Player = 0x00000000,
    /// gESpecialEntity_Focus    
    Focus = 0x00000001,
    /// gESpecialEntity_Interact
    Interact = 0x00000002,
    /// gESpecialEntity_Trader   
    Trader = 0x00000003,
    /// gESpecialEntity_DialogNPC
    DialogNPC = 0x00000004,
}

// gCNPC_PS.Species
/// gESpecies
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Species {
    /// gESpecies_None         
    None = 0x00000000,
    /// gESpecies_Human        
    Human = 0x00000001,
    /// gESpecies_Lizard       
    Lizard = 0x00000002,
    /// gESpecies_Brontok      
    Brontok = 0x00000003,
    /// gESpecies_Wolf_Tame    
    WolfTame = 0x00000004,
    /// gESpecies_Lurker       
    Lurker = 0x00000005,
    /// gESpecies_Ashbeast     
    Ashbeast = 0x00000006,
    /// gESpecies_Nautilus     
    Nautilus = 0x00000007,
    /// gESpecies_Dragonfly    
    Dragonfly = 0x00000008,
    /// gESpecies_Mantis       
    Mantis = 0x00000009,
    /// gESpecies_Scorpion     
    Scorpion = 0x0000000A,
    /// gESpecies_Skeleton     
    Skeleton = 0x0000000B,
    /// gESpecies_Swampmummy   
    Swampmummy = 0x0000000C,
    /// gESpecies_Rotworm      
    Rotworm = 0x0000000D,
    /// gESpecies_Skeleton_Tame
    SkeletonTame = 0x0000000E,
    /// gESpecies_Gnome        
    Gnome = 0x0000000F,
    /// gESpecies_Boar         
    Boar = 0x00000010,
    /// gESpecies_Wolf         
    Wolf = 0x00000011,
    /// gESpecies_Stingrat     
    Stingrat = 0x00000012,
    /// gESpecies_Vulture      
    Vulture = 0x00000013,
    /// gESpecies_Thundertail  
    Thundertail = 0x00000014,
    /// gESpecies_Ogre         
    Ogre = 0x00000015,
    /// gESpecies_Ogre_Tame    
    OgreTame = 0x00000016,
    /// gESpecies_Cow          
    Cow = 0x00000017,
    /// gESpecies_Pig          
    Pig = 0x00000018,
    /// gESpecies_Chicken      
    Chicken = 0x00000019,
    /// gESpecies_Ghost        
    Ghost = 0x0000001A,
    /// gESpecies_Count        
    Count = 0x0000001B,
}

// gCSpinButton.SpinType
/// gESpinButtonType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum SpinButtonType {
    /// gESpinButtonType_Prev
    Prev = 0x00000000,
    /// gESpinButtonType_Cycle
    Cycle = 0x00000001,
    /// gESpinButtonType_Next
    Next = 0x00000002,
}

// gCInventoryStack.Type
/// gEStackType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum StackType {
    /// gEStackType_Normal
    Normal = 0x00000000,
    /// gEStackType_Trade
    Trade = 0x00000001,
    /// gEStackType_Hidden
    Hidden = 0x00000002,
}

// gCStateGraphAction.EventType
/// gEStateGraphEventType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum StateGraphEventType {
    /// gEStateGraphEventType_None     
    None = 0x00000000,
    /// gEStateGraphEventType_Trigger  
    Trigger = 0x00000001,
    /// gEStateGraphEventType_Untrigger
    Untrigger = 0x00000002,
    /// gEStateGraphEventType_Touch    
    Touch = 0x00000003,
    /// gEStateGraphEventType_Untouch  
    Untouch = 0x00000004,
    /// gEStateGraphEventType_Damage   
    Damage = 0x00000005,
}

// gCSkillPicbox.ViewMode
/// gEViewMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ViewMode {
    /// gEViewMode_Name       
    Name = 0x00000000,
    /// gEViewMode_Description
    Description = 0x00000001,
    /// gEViewMode_Value      
    Value = 0x00000002,
    /// gEViewMode_Icon       
    Icon = 0x00000003,
    /// gEViewMode_IconValue  
    IconValue = 0x00000004,
    /// gEViewMode_NameValue  
    NameValue = 0x00000005,
}

// gCEquipPicbox2.ViewMode
/// gCEquipPicbox2_gEViewMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum EquipViewMode {
    /// gEViewMode_Value    
    Value = 0x00000002,
    /// gEViewMode_Icon     
    Icon = 0x00000003,
    /// gEViewMode_IconValue
    IconValue = 0x00000004,
    /// gEViewMode_Damage   
    Damage = 0x00000006,
}

// gCQuickPicbox2.ViewMode
/// gCQuickPicbox2_gEViewMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum QuickViewMode {
    /// gEViewMode_Value    
    Value = 0x00000002,
    /// gEViewMode_Icon     
    Icon = 0x00000003,
    /// gEViewMode_IconValue
    IconValue = 0x00000004,
    /// gEViewMode_Hold     
    Hold = 0x00000007,
}

// gCCombatAI.CombatWalkMode
// gCNavigation_PS.GuideWalkMode
// gCQuest.GuideWalkMode
/// gEWalkMode
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum WalkMode {
    /// gEWalkMode_Run   
    Run = 0x00000002,
    /// gEWalkMode_Sneak
    Sneak = 0x00000000,
    /// gEWalkMode_Walk  
    Walk = 0x00000001,
    /// gEWalkMode_Sprint
    Sprint = 0x00000003,
}

// gCWrittenStatic2.WrittenType
/// gEWrittenType
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum WrittenType {
    /// gEWrittenType_Invalid
    Invalid = 0xFFFFFFFF,
    /// gEWrittenType_Book   
    Book = 0x00000000,
    /// gEWrittenType_Letter
    Letter = 0x00000001,
    /// gEWrittenType_Recipe
    Recipe = 0x00000002,
    /// gEWrittenType_Map    
    Map = 0x00000003,
}

/// gEActionAxis
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ActionAxis {
    Undefined = 0,
    TurnLeftRight,
    TurnUpDown,
}

/// gEActionAxis
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum ActionKey {
    Undefined = 0,
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    TurnLeft,
    TurnRight,
    TurnUp,
    TurnDown,
    ZoomIn,
    ZoomOut,
    LookBack,
    Walk,
    WeaponMode,
    PrimaryAction,
    SecondaryAction,
    QuickUse0,
    QuickUse1,
    QuickUse2,
    QuickUse3,
    QuickUse4,
    QuickUse5,
    QuickUse6,
    QuickUse7,
    QuickUse8,
    QuickUse9,
    QuickBar,
    Inventory,
    Map,
    Log,
    Equipment,
    Escape,
    QuickSave,
    QuickLoad,
}
