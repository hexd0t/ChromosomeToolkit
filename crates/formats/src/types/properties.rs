pub mod enums;

use std::io::Write;

use serde::{Deserialize, Serialize};

use super::*;
use crate::archive::*;
use crate::binimport::BinImport;
use crate::error::*;
use crate::helpers::*;

/// bCProperty
#[derive(Debug, Deserialize, Serialize)]
pub struct Property {
    pub name: String,
    /// always 30 for R1
    pub version: u16,
    pub data: Box<PropData>,
}

impl Property {
    pub fn load<R: ArchiveReadTarget>(src: &mut R) -> Result<Self> {
        let name = src.read_str()?.to_string();
        let ty = src.read_str()?.to_string();
        let version = read_u16(src)?;
        assert_eq!(version, 30);
        let data_len = read_u32(src)? as usize;
        let data = Box::new(match ty.as_str() {
            "int" => {
                assert_eq!(data_len, 4);
                PropData::Int(read_i32(src)?)
            }
            "bool" => {
                assert_eq!(data_len, 1);
                PropData::Bool(read_u8(src)? != 0)
            }
            "short" => {
                assert_eq!(data_len, 2);
                PropData::Short(read_i16(src)?)
            }
            "float" => {
                assert_eq!(data_len, 4);
                PropData::Float(read_f32(src)?)
            }
            "long" => {
                assert_eq!(data_len, 4);
                PropData::Long(read_u32(src)?)
            }
            "char" => {
                assert_eq!(data_len, 1);
                PropData::Char(read_u8(src)?)
            }
            "bCVector2" => {
                assert_eq!(data_len, 2 * 4);
                PropData::Vector2(Vec2::load(src)?)
            }
            "bCVector" => {
                assert_eq!(data_len, 3 * 4);
                PropData::Vector3(Vec3::load(src)?)
            }
            "bCVector4" => {
                assert_eq!(data_len, 4 * 4);
                PropData::Vector4(Vec4::load(src)?)
            }
            "bCMatrix" => {
                assert_eq!(data_len, 4 * 4 * 4);
                PropData::Matrix(Mat4::load(src)?)
            }
            "bCQuaternion" => {
                assert_eq!(data_len, 4 * 4);
                PropData::Quaternion(Quat::load(src)?)
            }
            "bCGuid" => {
                assert_eq!(data_len, 16 + 4);
                PropData::Guid(PropertyId::load(src)?)
            }
            "bCString" => {
                assert_eq!(data_len, 2);
                PropData::String(src.read_str()?.to_string())
            }
            "bCFloatColor" => {
                assert_eq!(data_len, 16);
                PropData::FloatColor {
                    unknown: read_u32(src)?,
                    r: read_f32(src)?,
                    g: read_f32(src)?,
                    b: read_f32(src)?,
                }
            }
            "bCBox" => {
                assert_eq!(data_len, 6 * 4);
                PropData::BoundingBox(BoundingBox::load(src)?)
            }
            "eCEntityProxy" => {
                assert!(data_len == 3 || data_len == 3 + 20); //invalid or valid
                PropData::EntityProxy(EntityProxy::load(src, true)?)
            }
            "eCTemplateEntityProxy" => {
                assert!(data_len == 3 || data_len == 3 + 20); //invalid or valid
                PropData::TemplateEntityProxy(EntityProxy::load(src, true)?)
            }
            "gEDirection"
            | "bENoiseTurbulence"
            | "eCImageFilterRTBase_eCGfxShared_eEColorFormat"
            | "eCImageResource2_eCGfxShared_eEColorFormat"
            | "eEAnchorMode"
            | "eEAudioChannelFallOff"
            | "eEAudioChannelGroup"
            | "eEAudioEmitterMode"
            | "eEAudioEmitterShape"
            | "eEBillboardTargetMode"
            | "eEBoolOverwrite"
            | "eCGuiRadioButton2_eECheckState"
            | "eCGuiCheckBox2_eECheckState"
            | "eECollisionGroup"
            | "eECollisionShapeType"
            | "eEColorSrcCombinerType"
            | "eEColorSrcSampleTexRepeat"
            | "eEColorSrcSwitchRepeat"
            | "eECoordinateSystem"
            | "eEDistanceType"
            | "eEDock"
            | "eEDynamicLightEffect"
            | "eEFacingDirection"
            | "eEFresnelTerm"
            | "eEGuiCursorSize"
            | "eEIFOutputMode"
            | "eEIFSizeMode"
            | "eEIFTextureMode"
            | "eEImageBlend"
            | "eEImageLayerBlend"
            | "eELightingStyle"
            | "eEListView"
            | "eEListViewAlign"
            | "eEListViewIconSize"
            | "eEListViewItemLayout"
            | "eEListViewTileSize"
            | "eELocationShape"
            | "eELocationTarget"
            | "eEMoverPlayBackMode"
            | "eEOverlayMode"
            | "eEPhysicRangeType"
            | "eEPictureMode"
            | "eEPropertySetType"
            | "eEReflectType"
            | "eERigidbody_Flag"
            | "eERotationFrom"
            | "eEShaderMaterialBRDFType"
            | "eEShaderMaterialBlendMode"
            | "eEShaderMaterialTransformation"
            | "eEShaderMaterialVersion"
            | "eEShadowCasterType"
            | "eEShadowMaskIndex"
            | "eEShapeAABBAdapt"
            | "eEShapeGroup"
            | "eCMaterialResource2_eEShapeMaterial"
            | "eEShapeMaterial"
            | "eESplitImageStyle"
            | "eEStaticIlluminated"
            | "eEStripSpawning"
            | "eETexCoordSrcOscillatorType"
            | "eETexCoordSrcRotatorType"
            | "eETextAlign"
            | "eETextureDrawStyle"
            | "eETicSide"
            | "eEVegetationBrushColorFunction"
            | "eEVegetationBrushMode"
            | "eEVegetationBrushPlace"
            | "eEVegetationBrushProbabilityFunction"
            | "eEVegetationBrushShape"
            | "eEVegetationMeshShading"
            | "eEVelocityDirectionFrom"
            | "eEWeatherZoneOverwrite"
            | "eEWeatherZoneShape"
            | "gEAIMode"
            | "gEAchievementViewMode"
            | "gEAlignToTarget"
            | "gEAmbientAction"
            | "gEAmountType"
            | "gEAnchorType"
            | "gEAniState"
            | "gCCombatMoveStumble_gEAniState"
            | "gEArenaStatus"
            | "gEAttitude"
            | "gEBoostTarget"
            | "gEBraveryOverride"
            | "gCCombatMoveMelee_gECombatAction"
            | "gCCombatMoveScriptState_gECombatAction"
            | "gECombatAttackStumble"
            | "gECombatComboParade"
            | "gECombatFightAIMode"
            | "gECombatHitDirection"
            | "gECombatMode"
            | "gCCombatMoveMelee_gECombatMove"
            | "gCCombatMoveMelee2_gECombatMove"
            | "gECombatMoveSide"
            | "gECombatParadeType"
            | "gECombatPhaseType"
            | "gECombatPose"
            | "gEComment"
            | "gECompareOperation"
            | "gECrime"
            | "gEDamageCalculationType"
            | "gEDamageType"
            | "gEDoorStatus"
            | "gEEffectDecayMode"
            | "gEEffectKillRange"
            | "gEEffectLink"
            | "gEEffectLoopMode"
            | "gEEffectScriptOtherType"
            | "gEEffectScriptParamType"
            | "gEEffectStopMode"
            | "gEEffectTargetMode"
            | "gEEntityType"
            | "gCEquipPicbox2_gEEquipSlot"
            | "gEEquipSlot"
            | "gEFight"
            | "gEFlightPathType"
            | "gEFocusNameType"
            | "gEFocusPriority"
            | "gEFocusSource"
            | "gEGUIFilterType"
            | "gEGammaRamp"
            | "gEGender"
            | "gEGuardStatus"
            | "gCNPC_PS_gEGuardStatus"
            | "gEGuild"
            | "gCNPC_PS_gEGuild"
            | "gEHitDirection"
            | "gEHudPage"
            | "gCPageTimerProgressBar_gEHudPage"
            | "gEIcon"
            | "gEInfoCondType"
            | "gCInfoConditionQuestStatus_gEInfoCondType"
            | "gEInfoGesture"
            | "gEInfoLocation"
            | "gEInfoNPCStatus"
            | "gEInfoNPCType"
            | "gEInfoType"
            | "gEInfoView"
            | "gCSkillInfo_gEInfoView"
            | "gCDialogInfo_gCDialogInfo_gEInfoView"
            | "gCHintStatic_gEInfoView"
            | "gEInteractionType"
            | "SPECIAL_gEInteractionUseType"
            | "gEInteractionUseType"
            | "gEItemCategory"
            | "gEItemHoldType"
            | "gEItemUseType"
            | "gCCombatWeaponConfig_gEItemUseType"
            | "gELockStatus"
            | "gEMiscInfo"
            | "gCMiscProgressBar_gEMiscInfo"
            | "gEMouseAxis"
            | "gENavObstacleType"
            | "gENavTestResult"
            | "gEOtherType"
            | "gCLootStatic2_gEPageMode"
            | "gCHudPage2_gEPageMode"
            | "gEPageMode"
            | "gEPaintArea"
            | "gEPartyMemberType"
            | "gEQuestActor"
            | "gEQuestStatus"
            | "gEQuestType"
            | "gEQuickSlot"
            | "gEReason"
            | "gERecipeCategory"
            | "gEScrollStart"
            | "gESession_State"
            | "gESkill"
            | "gCSkillValueBase_gESkill"
            | "gCSkillProgressBar_gESkill"
            | "gESkillModifier"
            | "gESpecialEntity"
            | "gESpecies"
            | "gESpinButtonType"
            | "gEStackType"
            | "gEStateGraphEventType"
            | "gEViewMode"
            | "gCEquipPicbox2_gEViewMode"
            | "gCQuickPicbox2_gEViewMode"
            | "gEWalkMode"
            | "gEWrittenType" => {
                assert_eq!(data_len, 6); //enum header u16 + enum u32
                PropData::Enum(Self::load_enum(src, ty.as_str(), true)?)
            }
            _ if ty.starts_with("bTPropertyContainer<enum ") => {
                assert_eq!(data_len, 6); //enum header u16 + enum u32
                const NAME_START: usize = "bTPropertyContainer<enum ".len();
                let name_end = ty.len() - ">".len();
                PropData::ContainerEnum(Self::load_enum(src, &ty[NAME_START..name_end], true)?)
            }
            _ => {
                println!("Unknown property type '{ty}'");
                let mut data = vec![0u8; data_len];
                src.read_exact(&mut data)?;
                PropData::Buffer(PropBuffer { ty, data })
            }
        });

        Ok(Self {
            name,
            version,
            data,
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let mut block = TempWriteTarget::new(dst);
        //needed for lifetime reasons if the match needs to calculate the type name:
        let dynamic_type: String;

        let ty = match &*self.data {
            PropData::Int(v) => {
                write_i32(&mut block, *v)?;
                "int"
            }
            PropData::Bool(true) => {
                write_u8(&mut block, 1)?;
                "bool"
            }
            PropData::Bool(false) => {
                write_u8(&mut block, 0)?;
                "bool"
            }
            PropData::Short(v) => {
                write_i16(&mut block, *v)?;
                "short"
            }
            PropData::Float(v) => {
                write_f32(&mut block, *v)?;
                "float"
            }
            PropData::Long(v) => {
                write_u32(&mut block, *v)?;
                "long"
            }
            PropData::Char(v) => {
                write_u8(&mut block, *v)?;
                "char"
            }
            PropData::Vector2(vector2) => {
                vector2.save(&mut block)?;
                "bCVector2"
            }
            PropData::Vector3(vector3) => {
                vector3.save(&mut block)?;
                "bCVector"
            }
            PropData::Vector4(vector4) => {
                vector4.save(&mut block)?;
                "bCVector4"
            }
            PropData::Matrix(matrix) => {
                matrix.save(&mut block)?;
                "bCMatrix"
            }
            PropData::Quaternion(quat) => {
                quat.save(&mut block)?;
                "bCQuaternion"
            }
            PropData::String(str) => {
                block.write_str(str)?;
                "bCString"
            }
            PropData::ImageOrMaterialResourceString(_) => todo!(),
            PropData::LocString(_) => todo!(),
            PropData::ImageResourceString(_) => todo!(),
            PropData::LetterLocString(_) => todo!(),
            PropData::SpeedTreeResourceString(_) => todo!(),
            PropData::BookLocString(_) => todo!(),
            PropData::MeshResourceString(_) => todo!(),
            PropData::InfoLocString(_) => todo!(),
            PropData::QuestLocString(_) => todo!(),
            PropData::TipLocString(_) => todo!(),
            PropData::NPCInfoLocString(_) => todo!(),
            PropData::FloatColor { unknown, r, g, b } => {
                write_u32(&mut block, *unknown)?;
                write_f32(&mut block, *r)?;
                write_f32(&mut block, *g)?;
                write_f32(&mut block, *b)?;
                "bCFloatColor"
            }
            PropData::EntityProxy(entity_proxy) => {
                entity_proxy.save(&mut block, true)?;
                "eCEntityProxy"
            }
            PropData::TemplateEntityProxy(entity_proxy) => {
                entity_proxy.save(&mut block, true)?;
                "eCTemplateEntityProxy"
            }
            PropData::BoundingBox(bounding_box) => {
                bounding_box.save(&mut block)?;
                "bCBox"
            }
            PropData::Buffer(prop_buffer) => {
                block.write_all(&prop_buffer.data)?;
                prop_buffer.ty.as_str()
            }
            PropData::Guid(guid) => {
                guid.save(&mut block)?;
                "bCGuid"
            }
            PropData::Enum(prop_enum) => Self::save_enum(&mut block, prop_enum, true)?,
            PropData::ContainerEnum(prop_enum) => {
                let name = Self::save_enum(&mut block, prop_enum, true)?;
                dynamic_type = format!("bTPropertyContainer<enum {name}>");
                dynamic_type.as_str()
            }
        };

        let prop_data = block.finish();

        dst.write_str(self.name.as_str())?;
        dst.write_str(ty)?;
        write_u16(dst, self.version)?;

        write_u32(dst, prop_data.len() as u32)?;
        dst.write_all(&prop_data)?;
        Ok(())
    }

    /// read_header = true for everything but enums in a bTObjArray
    pub fn load_enum<R: ArchiveReadTarget>(
        src: &mut R,
        enum_name: &str,
        read_header: bool,
    ) -> Result<PropEnum> {
        if read_header {
            let head = read_u16(src)?;
            assert_eq!(head, 201);
        }
        let discriminant = read_u32(src)?;
        Ok(match enum_name {
            "gEDirection" => PropEnum::Direction(enums::Direction::try_from(discriminant)?),
            "bENoiseTurbulence" => {
                PropEnum::NoiseTurbulence(enums::NoiseTurbulence::try_from(discriminant)?)
            }
            "eCImageFilterRTBase_eCGfxShared_eEColorFormat" => PropEnum::ImageFilterRTColorFormat(
                enums::ImageFilterRTColorFormat::try_from(discriminant)?,
            ),
            "eCImageResource2_eCGfxShared_eEColorFormat" => PropEnum::ImageResource2ColorFormat(
                enums::ImageResource2ColorFormat::try_from(discriminant)?,
            ),
            "eEAnchorMode" => PropEnum::AnchorMode(enums::AnchorMode::try_from(discriminant)?),
            "eEAudioChannelFallOff" => {
                PropEnum::AudioChannelFallOff(enums::AudioChannelFallOff::try_from(discriminant)?)
            }
            "eEAudioChannelGroup" => {
                PropEnum::AudioChannelGroup(enums::AudioChannelGroup::try_from(discriminant)?)
            }
            "eEAudioEmitterMode" => {
                PropEnum::AudioEmitterMode(enums::AudioEmitterMode::try_from(discriminant)?)
            }
            "eEAudioEmitterShape" => {
                PropEnum::AudioEmitterShape(enums::AudioEmitterShape::try_from(discriminant)?)
            }
            "eEBillboardTargetMode" => {
                PropEnum::BillboardTargetMode(enums::BillboardTargetMode::try_from(discriminant)?)
            }
            "eEBoolOverwrite" => {
                PropEnum::BoolOverwrite(enums::BoolOverwrite::try_from(discriminant)?)
            }
            "eCGuiRadioButton2_eECheckState" => {
                PropEnum::RadioCheckState(enums::RadioCheckState::try_from(discriminant)?)
            }
            "eCGuiCheckBox2_eECheckState" => {
                PropEnum::CheckBoxCheckState(enums::CheckBoxCheckState::try_from(discriminant)?)
            }
            "eECollisionGroup" => {
                PropEnum::CollisionGroup(enums::CollisionGroup::try_from(discriminant)?)
            }
            "eECollisionShapeType" => {
                PropEnum::CollisionShapeType(enums::CollisionShapeType::try_from(discriminant)?)
            }
            "eEColorSrcCombinerType" => {
                PropEnum::ColorSrcCombinerType(enums::ColorSrcCombinerType::try_from(discriminant)?)
            }
            "eEColorSrcSampleTexRepeat" => PropEnum::ColorSrcSampleTexRepeat(
                enums::ColorSrcSampleTexRepeat::try_from(discriminant)?,
            ),
            "eEColorSrcSwitchRepeat" => {
                PropEnum::ColorSrcSwitchRepeat(enums::ColorSrcSwitchRepeat::try_from(discriminant)?)
            }
            "eECoordinateSystem" => {
                PropEnum::CoordinateSystem(enums::CoordinateSystem::try_from(discriminant)?)
            }
            "eEDistanceType" => {
                PropEnum::DistanceType(enums::DistanceType::try_from(discriminant)?)
            }
            "eEDock" => PropEnum::Dock(enums::Dock::try_from(discriminant)?),
            "eEDynamicLightEffect" => {
                PropEnum::DynamicLightEffect(enums::DynamicLightEffect::try_from(discriminant)?)
            }
            "eEFacingDirection" => {
                PropEnum::FacingDirection(enums::FacingDirection::try_from(discriminant)?)
            }
            "eEFresnelTerm" => PropEnum::FresnelTerm(enums::FresnelTerm::try_from(discriminant)?),
            "eEGuiCursorSize" => {
                PropEnum::GuiCursorSize(enums::GuiCursorSize::try_from(discriminant)?)
            }
            "eEIFOutputMode" => {
                PropEnum::IFOutputMode(enums::IFOutputMode::try_from(discriminant)?)
            }
            "eEIFSizeMode" => PropEnum::IFSizeMode(enums::IFSizeMode::try_from(discriminant)?),
            "eEIFTextureMode" => {
                PropEnum::IFTextureMode(enums::IFTextureMode::try_from(discriminant)?)
            }
            "eEImageBlend" => PropEnum::ImageBlend(enums::ImageBlend::try_from(discriminant)?),
            "eEImageLayerBlend" => {
                PropEnum::ImageLayerBlend(enums::ImageLayerBlend::try_from(discriminant)?)
            }
            "eELightingStyle" => {
                PropEnum::LightingStyle(enums::LightingStyle::try_from(discriminant)?)
            }
            "eEListView" => PropEnum::ListView(enums::ListView::try_from(discriminant)?),
            "eEListViewAlign" => {
                PropEnum::ListViewAlign(enums::ListViewAlign::try_from(discriminant)?)
            }
            "eEListViewIconSize" => {
                PropEnum::ListViewIconSize(enums::ListViewIconSize::try_from(discriminant)?)
            }
            "eEListViewItemLayout" => {
                PropEnum::ListViewItemLayout(enums::ListViewItemLayout::try_from(discriminant)?)
            }
            "eEListViewTileSize" => {
                PropEnum::ListViewTileSize(enums::ListViewTileSize::try_from(discriminant)?)
            }
            "eELocationShape" => {
                PropEnum::LocationShape(enums::LocationShape::try_from(discriminant)?)
            }
            "eELocationTarget" => {
                PropEnum::LocationTarget(enums::LocationTarget::try_from(discriminant)?)
            }
            "eEMoverPlayBackMode" => {
                PropEnum::MoverPlayBackMode(enums::MoverPlayBackMode::try_from(discriminant)?)
            }
            "eEOverlayMode" => PropEnum::OverlayMode(enums::OverlayMode::try_from(discriminant)?),
            "eEPhysicRangeType" => {
                PropEnum::PhysicRangeType(enums::PhysicRangeType::try_from(discriminant)?)
            }
            "eEPictureMode" => PropEnum::PictureMode(enums::PictureMode::try_from(discriminant)?),
            "eEPropertySetType" => {
                PropEnum::PropertySetType(enums::PropertySetType::try_from(discriminant)?)
            }
            "eEReflectType" => PropEnum::ReflectType(enums::ReflectType::try_from(discriminant)?),
            "eERigidbody_Flag" => {
                PropEnum::RigidbodyFlag(enums::RigidbodyFlag::try_from(discriminant)?)
            }
            "eERotationFrom" => {
                PropEnum::RotationFrom(enums::RotationFrom::try_from(discriminant)?)
            }
            "eEShaderMaterialBRDFType" => PropEnum::ShaderMaterialBRDFType(
                enums::ShaderMaterialBRDFType::try_from(discriminant)?,
            ),
            "eEShaderMaterialBlendMode" => PropEnum::ShaderMaterialBlendMode(
                enums::ShaderMaterialBlendMode::try_from(discriminant)?,
            ),
            "eEShaderMaterialTransformation" => PropEnum::ShaderMaterialTransformation(
                enums::ShaderMaterialTransformation::try_from(discriminant)?,
            ),
            "eEShaderMaterialVersion" => PropEnum::ShaderMaterialVersion(
                enums::ShaderMaterialVersion::try_from(discriminant)?,
            ),
            "eEShadowCasterType" => {
                PropEnum::ShadowCasterType(enums::ShadowCasterType::try_from(discriminant)?)
            }
            "eEShadowMaskIndex" => {
                PropEnum::ShadowMaskIndex(enums::ShadowMaskIndex::try_from(discriminant)?)
            }
            "eEShapeAABBAdapt" => {
                PropEnum::ShapeAABBAdapt(enums::ShapeAABBAdapt::try_from(discriminant)?)
            }
            "eEShapeGroup" => PropEnum::ShapeGroup(enums::ShapeGroup::try_from(discriminant)?),
            "eCMaterialResource2_eEShapeMaterial" => {
                PropEnum::ShapeMaterial(enums::ShapeMaterial::try_from(discriminant)?)
            }
            "eEShapeMaterial" => {
                PropEnum::ShapeMaterial(enums::ShapeMaterial::try_from(discriminant)?)
            }
            "eESplitImageStyle" => {
                PropEnum::SplitImageStyle(enums::SplitImageStyle::try_from(discriminant)?)
            }
            "eEStaticIlluminated" => {
                PropEnum::StaticIlluminated(enums::StaticIlluminated::try_from(discriminant)?)
            }
            "eEStripSpawning" => {
                PropEnum::StripSpawning(enums::StripSpawning::try_from(discriminant)?)
            }
            "eETexCoordSrcOscillatorType" => PropEnum::TexCoordSrcOscillatorType(
                enums::TexCoordSrcOscillatorType::try_from(discriminant)?,
            ),
            "eETexCoordSrcRotatorType" => PropEnum::TexCoordSrcRotatorType(
                enums::TexCoordSrcRotatorType::try_from(discriminant)?,
            ),
            "eETextAlign" => PropEnum::TextAlign(enums::TextAlign::try_from(discriminant)?),
            "eETextureDrawStyle" => {
                PropEnum::TextureDrawStyle(enums::TextureDrawStyle::try_from(discriminant)?)
            }
            "eETicSide" => PropEnum::TicSide(enums::TicSide::try_from(discriminant)?),
            "eEVegetationBrushColorFunction" => PropEnum::VegetationBrushColorFunction(
                enums::VegetationBrushColorFunction::try_from(discriminant)?,
            ),
            "eEVegetationBrushMode" => {
                PropEnum::VegetationBrushMode(enums::VegetationBrushMode::try_from(discriminant)?)
            }
            "eEVegetationBrushPlace" => {
                PropEnum::VegetationBrushPlace(enums::VegetationBrushPlace::try_from(discriminant)?)
            }
            "eEVegetationBrushProbabilityFunction" => PropEnum::VegetationBrushProbabilityFunction(
                enums::VegetationBrushProbabilityFunction::try_from(discriminant)?,
            ),
            "eEVegetationBrushShape" => {
                PropEnum::VegetationBrushShape(enums::VegetationBrushShape::try_from(discriminant)?)
            }
            "eEVegetationMeshShading" => PropEnum::VegetationMeshShading(
                enums::VegetationMeshShading::try_from(discriminant)?,
            ),
            "eEVelocityDirectionFrom" => PropEnum::VelocityDirectionFrom(
                enums::VelocityDirectionFrom::try_from(discriminant)?,
            ),
            "eEWeatherZoneOverwrite" => {
                PropEnum::WeatherZoneOverwrite(enums::WeatherZoneOverwrite::try_from(discriminant)?)
            }
            "eEWeatherZoneShape" => {
                PropEnum::WeatherZoneShape(enums::WeatherZoneShape::try_from(discriminant)?)
            }
            "gEAIMode" => PropEnum::AIMode(enums::AIMode::try_from(discriminant)?),
            "gEAchievementViewMode" => {
                PropEnum::AchievementViewMode(enums::AchievementViewMode::try_from(discriminant)?)
            }
            "gEAlignToTarget" => {
                PropEnum::AlignToTarget(enums::AlignToTarget::try_from(discriminant)?)
            }
            "gEAmbientAction" => {
                PropEnum::AmbientAction(enums::AmbientAction::try_from(discriminant)?)
            }
            "gEAmountType" => PropEnum::AmountType(enums::AmountType::try_from(discriminant)?),
            "gEAnchorType" => PropEnum::AnchorType(enums::AnchorType::try_from(discriminant)?),
            "gEAniState" => PropEnum::AniState(enums::AniState::try_from(discriminant)?),
            "gCCombatMoveStumble_gEAniState" => PropEnum::CombatMoveStumbleAniState(
                enums::CombatMoveStumbleAniState::try_from(discriminant)?,
            ),
            "gEArenaStatus" => PropEnum::ArenaStatus(enums::ArenaStatus::try_from(discriminant)?),
            "gEAttitude" => PropEnum::Attitude(enums::Attitude::try_from(discriminant)?),
            "gEBoostTarget" => PropEnum::BoostTarget(enums::BoostTarget::try_from(discriminant)?),
            "gEBraveryOverride" => {
                PropEnum::BraveryOverride(enums::BraveryOverride::try_from(discriminant)?)
            }
            "gCCombatMoveMelee_gECombatAction" => {
                PropEnum::MeleeCombatAction(enums::MeleeCombatAction::try_from(discriminant)?)
            }
            "gCCombatMoveScriptState_gECombatAction" => {
                PropEnum::MoveCombatAction(enums::MoveCombatAction::try_from(discriminant)?)
            }
            "gECombatAttackStumble" => {
                PropEnum::CombatAttackStumble(enums::CombatAttackStumble::try_from(discriminant)?)
            }
            "gECombatComboParade" => {
                PropEnum::CombatComboParade(enums::CombatComboParade::try_from(discriminant)?)
            }
            "gECombatFightAIMode" => {
                PropEnum::CombatFightAIMode(enums::CombatFightAIMode::try_from(discriminant)?)
            }
            "gECombatHitDirection" => {
                PropEnum::CombatHitDirection(enums::CombatHitDirection::try_from(discriminant)?)
            }
            "gECombatMode" => PropEnum::CombatMode(enums::CombatMode::try_from(discriminant)?),
            "gCCombatMoveMelee_gECombatMove" => {
                PropEnum::MeleeCombatMove(enums::MeleeCombatMove::try_from(discriminant)?)
            }
            "gCCombatMoveMelee2_gECombatMove" => {
                PropEnum::Melee2CombatMove(enums::Melee2CombatMove::try_from(discriminant)?)
            }
            "gECombatMoveSide" => {
                PropEnum::CombatMoveSide(enums::CombatMoveSide::try_from(discriminant)?)
            }
            "gECombatParadeType" => {
                PropEnum::CombatParadeType(enums::CombatParadeType::try_from(discriminant)?)
            }
            "gECombatPhaseType" => {
                PropEnum::CombatPhaseType(enums::CombatPhaseType::try_from(discriminant)?)
            }
            "gECombatPose" => PropEnum::CombatPose(enums::CombatPose::try_from(discriminant)?),
            "gEComment" => PropEnum::Comment(enums::Comment::try_from(discriminant)?),
            "gECompareOperation" => {
                PropEnum::CompareOperation(enums::CompareOperation::try_from(discriminant)?)
            }
            "gECrime" => PropEnum::Crime(enums::Crime::try_from(discriminant)?),
            "gEDamageCalculationType" => PropEnum::DamageCalculationType(
                enums::DamageCalculationType::try_from(discriminant)?,
            ),
            "gEDamageType" => PropEnum::DamageType(enums::DamageType::try_from(discriminant)?),
            "gEDoorStatus" => PropEnum::DoorStatus(enums::DoorStatus::try_from(discriminant)?),
            "gEEffectDecayMode" => {
                PropEnum::EffectDecayMode(enums::EffectDecayMode::try_from(discriminant)?)
            }
            "gEEffectKillRange" => {
                PropEnum::EffectKillRange(enums::EffectKillRange::try_from(discriminant)?)
            }
            "gEEffectLink" => PropEnum::EffectLink(enums::EffectLink::try_from(discriminant)?),
            "gEEffectLoopMode" => {
                PropEnum::EffectLoopMode(enums::EffectLoopMode::try_from(discriminant)?)
            }
            "gEEffectScriptOtherType" => PropEnum::EffectScriptOtherType(
                enums::EffectScriptOtherType::try_from(discriminant)?,
            ),
            "gEEffectScriptParamType" => PropEnum::EffectScriptParamType(
                enums::EffectScriptParamType::try_from(discriminant)?,
            ),
            "gEEffectStopMode" => {
                PropEnum::EffectStopMode(enums::EffectStopMode::try_from(discriminant)?)
            }
            "gEEffectTargetMode" => {
                PropEnum::EffectTargetMode(enums::EffectTargetMode::try_from(discriminant)?)
            }
            "gEEntityType" => PropEnum::EntityType(enums::EntityType::try_from(discriminant)?),
            "gCEquipPicbox2_gEEquipSlot" => {
                PropEnum::Picbox2EquipSlot(enums::Picbox2EquipSlot::try_from(discriminant)?)
            }
            "gEEquipSlot" => PropEnum::EquipSlot(enums::EquipSlot::try_from(discriminant)?),
            "gEFight" => PropEnum::Fight(enums::Fight::try_from(discriminant)?),
            "gEFlightPathType" => {
                PropEnum::FlightPathType(enums::FlightPathType::try_from(discriminant)?)
            }
            "gEFocusNameType" => {
                PropEnum::FocusNameType(enums::FocusNameType::try_from(discriminant)?)
            }
            "gEFocusPriority" => {
                PropEnum::FocusPriority(enums::FocusPriority::try_from(discriminant)?)
            }
            "gEFocusSource" => PropEnum::FocusSource(enums::FocusSource::try_from(discriminant)?),
            "gEGUIFilterType" => {
                PropEnum::GUIFilterType(enums::GUIFilterType::try_from(discriminant)?)
            }
            "gEGammaRamp" => PropEnum::GammaRamp(enums::GammaRamp::try_from(discriminant)?),
            "gEGender" => PropEnum::Gender(enums::Gender::try_from(discriminant)?),
            "gEGuardStatus" => PropEnum::GuardStatus(enums::GuardStatus::try_from(discriminant)?),
            "gCNPC_PS_gEGuardStatus" => {
                PropEnum::NpcGuardStatus(enums::NpcGuardStatus::try_from(discriminant)?)
            }
            "gEGuild" => PropEnum::Guild(enums::Guild::try_from(discriminant)?),
            "gCNPC_PS_gEGuild" => PropEnum::NpcGuild(enums::NpcGuild::try_from(discriminant)?),
            "gEHitDirection" => {
                PropEnum::HitDirection(enums::HitDirection::try_from(discriminant)?)
            }
            "gEHudPage" => PropEnum::HudPage(enums::HudPage::try_from(discriminant)?),
            "gCPageTimerProgressBar_gEHudPage" => {
                PropEnum::HudPageProgessBar(enums::HudPageProgessBar::try_from(discriminant)?)
            }
            "gEIcon" => PropEnum::Icon(enums::Icon::try_from(discriminant)?),
            "gEInfoCondType" => {
                PropEnum::InfoCondType(enums::InfoCondType::try_from(discriminant)?)
            }
            "gCInfoConditionQuestStatus_gEInfoCondType" => {
                PropEnum::QuestInfoCondType(enums::QuestInfoCondType::try_from(discriminant)?)
            }
            "gEInfoGesture" => PropEnum::InfoGesture(enums::InfoGesture::try_from(discriminant)?),
            "gEInfoLocation" => {
                PropEnum::InfoLocation(enums::InfoLocation::try_from(discriminant)?)
            }
            "gEInfoNPCStatus" => {
                PropEnum::InfoNPCStatus(enums::InfoNPCStatus::try_from(discriminant)?)
            }
            "gEInfoNPCType" => PropEnum::InfoNPCType(enums::InfoNPCType::try_from(discriminant)?),
            "gEInfoType" => PropEnum::InfoType(enums::InfoType::try_from(discriminant)?),
            "gEInfoView" => PropEnum::InfoView(enums::InfoView::try_from(discriminant)?),
            "gCSkillInfo_gEInfoView" => {
                PropEnum::SkillInfoView(enums::SkillInfoView::try_from(discriminant)?)
            }
            "gCDialogInfo_gCDialogInfo_gEInfoView" => {
                PropEnum::DialogInfoView(enums::DialogInfoView::try_from(discriminant)?)
            }
            "gCHintStatic_gEInfoView" => {
                PropEnum::HintInfoView(enums::HintInfoView::try_from(discriminant)?)
            }
            "gEInteractionType" => {
                PropEnum::InteractionType(enums::InteractionType::try_from(discriminant)?)
            }
            "SPECIAL_gEInteractionUseType" => PropEnum::SpecialInteractionUseType(
                enums::SpecialInteractionUseType::try_from(discriminant)?,
            ),
            "gEInteractionUseType" => {
                PropEnum::InteractionUseType(enums::InteractionUseType::try_from(discriminant)?)
            }
            "gEItemCategory" => {
                PropEnum::ItemCategory(enums::ItemCategory::try_from(discriminant)?)
            }
            "gEItemHoldType" => {
                PropEnum::ItemHoldType(enums::ItemHoldType::try_from(discriminant)?)
            }
            "gEItemUseType" => PropEnum::ItemUseType(enums::ItemUseType::try_from(discriminant)?),
            "gCCombatWeaponConfig_gEItemUseType" => {
                PropEnum::WeaponItemUseType(enums::WeaponItemUseType::try_from(discriminant)?)
            }
            "gELockStatus" => PropEnum::LockStatus(enums::LockStatus::try_from(discriminant)?),
            "gEMiscInfo" => PropEnum::MiscInfo(enums::MiscInfo::try_from(discriminant)?),
            "gCMiscProgressBar_gEMiscInfo" => {
                PropEnum::ProgressBarMiscInfo(enums::ProgressBarMiscInfo::try_from(discriminant)?)
            }
            "gEMouseAxis" => PropEnum::MouseAxis(enums::MouseAxis::try_from(discriminant)?),
            "gENavObstacleType" => {
                PropEnum::NavObstacleType(enums::NavObstacleType::try_from(discriminant)?)
            }
            "gENavTestResult" => {
                PropEnum::NavTestResult(enums::NavTestResult::try_from(discriminant)?)
            }
            "gEOtherType" => PropEnum::OtherType(enums::OtherType::try_from(discriminant)?),
            "gCLootStatic2_gEPageMode" => {
                PropEnum::LootPageMode(enums::LootPageMode::try_from(discriminant)?)
            }
            "gCHudPage2_gEPageMode" => {
                PropEnum::HudPageMode(enums::HudPageMode::try_from(discriminant)?)
            }
            "gEPageMode" => PropEnum::PageMode(enums::PageMode::try_from(discriminant)?),
            "gEPaintArea" => PropEnum::PaintArea(enums::PaintArea::try_from(discriminant)?),
            "gEPartyMemberType" => {
                PropEnum::PartyMemberType(enums::PartyMemberType::try_from(discriminant)?)
            }
            "gEQuestActor" => PropEnum::QuestActor(enums::QuestActor::try_from(discriminant)?),
            "gEQuestStatus" => PropEnum::QuestStatus(enums::QuestStatus::try_from(discriminant)?),
            "gEQuestType" => PropEnum::QuestType(enums::QuestType::try_from(discriminant)?),
            "gEQuickSlot" => PropEnum::QuickSlot(enums::QuickSlot::try_from(discriminant)?),
            "gEReason" => PropEnum::Reason(enums::Reason::try_from(discriminant)?),
            "gERecipeCategory" => {
                PropEnum::RecipeCategory(enums::RecipeCategory::try_from(discriminant)?)
            }
            "gEScrollStart" => PropEnum::ScrollStart(enums::ScrollStart::try_from(discriminant)?),
            "gESession_State" => {
                PropEnum::SessionState(enums::SessionState::try_from(discriminant)?)
            }
            "gESkill" => PropEnum::Skill(enums::Skill::try_from(discriminant)?),
            "gCSkillValueBase_gESkill" => {
                PropEnum::SkillValueBaseSkill(enums::SkillValueBaseSkill::try_from(discriminant)?)
            }
            "gCSkillProgressBar_gESkill" => {
                PropEnum::ProgressBarSkill(enums::ProgressBarSkill::try_from(discriminant)?)
            }
            "gESkillModifier" => {
                PropEnum::SkillModifier(enums::SkillModifier::try_from(discriminant)?)
            }
            "gESpecialEntity" => {
                PropEnum::SpecialEntity(enums::SpecialEntity::try_from(discriminant)?)
            }
            "gESpecies" => PropEnum::Species(enums::Species::try_from(discriminant)?),
            "gESpinButtonType" => {
                PropEnum::SpinButtonType(enums::SpinButtonType::try_from(discriminant)?)
            }
            "gEStackType" => PropEnum::StackType(enums::StackType::try_from(discriminant)?),
            "gEStateGraphEventType" => {
                PropEnum::StateGraphEventType(enums::StateGraphEventType::try_from(discriminant)?)
            }
            "gEViewMode" => PropEnum::ViewMode(enums::ViewMode::try_from(discriminant)?),
            "gCEquipPicbox2_gEViewMode" => {
                PropEnum::EquipViewMode(enums::EquipViewMode::try_from(discriminant)?)
            }
            "gCQuickPicbox2_gEViewMode" => {
                PropEnum::QuickViewMode(enums::QuickViewMode::try_from(discriminant)?)
            }
            "gEWalkMode" => PropEnum::WalkMode(enums::WalkMode::try_from(discriminant)?),
            "gEWrittenType" => PropEnum::WrittenType(enums::WrittenType::try_from(discriminant)?),

            _ => {
                println!("Unknown enum '{enum_name}'");
                PropEnum::Unknown(UnknownEnum {
                    name: enum_name.to_string(),
                    val: discriminant,
                })
            }
        })
    }

    /// write_header = true for everything but enums in a bTObjArray
    pub fn save_enum<'a, W: ArchiveWriteTarget>(
        dst: &mut W,
        _enum: &'a PropEnum,
        write_header: bool,
    ) -> Result<&'a str> {
        if write_header {
            write_u16(dst, 201)?;
        }
        let (name, discriminant) = match _enum {
            PropEnum::Direction(e) => ("gEDirection", (*e).into()),
            PropEnum::NoiseTurbulence(e) => ("bENoiseTurbulence", (*e).into()),
            PropEnum::ImageFilterRTColorFormat(e) => {
                ("eCImageFilterRTBase_eCGfxShared_eEColorFormat", (*e).into())
            }
            PropEnum::ImageResource2ColorFormat(e) => {
                ("eCImageResource2_eCGfxShared_eEColorFormat", (*e).into())
            }
            PropEnum::AnchorMode(e) => ("eEAnchorMode", (*e).into()),
            PropEnum::AudioChannelFallOff(e) => ("eEAudioChannelFallOff", (*e).into()),
            PropEnum::AudioChannelGroup(e) => ("eEAudioChannelGroup", (*e).into()),
            PropEnum::AudioEmitterMode(e) => ("eEAudioEmitterMode", (*e).into()),
            PropEnum::AudioEmitterShape(e) => ("eEAudioEmitterShape", (*e).into()),
            PropEnum::BillboardTargetMode(e) => ("eEBillboardTargetMode", (*e).into()),
            PropEnum::BoolOverwrite(e) => ("eEBoolOverwrite", (*e).into()),
            PropEnum::RadioCheckState(e) => ("eCGuiRadioButton2_eECheckState", (*e).into()),
            PropEnum::CheckBoxCheckState(e) => ("eCGuiCheckBox2_eECheckState", (*e).into()),
            PropEnum::CollisionGroup(e) => ("eECollisionGroup", (*e).into()),
            PropEnum::CollisionShapeType(e) => ("eECollisionShapeType", (*e).into()),
            PropEnum::ColorSrcCombinerType(e) => ("eEColorSrcCombinerType", (*e).into()),
            PropEnum::ColorSrcSampleTexRepeat(e) => ("eEColorSrcSampleTexRepeat", (*e).into()),
            PropEnum::ColorSrcSwitchRepeat(e) => ("eEColorSrcSwitchRepeat", (*e).into()),
            PropEnum::CoordinateSystem(e) => ("eECoordinateSystem", (*e).into()),
            PropEnum::DistanceType(e) => ("eEDistanceType", (*e).into()),
            PropEnum::Dock(e) => ("eEDock", (*e).into()),
            PropEnum::DynamicLightEffect(e) => ("eEDynamicLightEffect", (*e).into()),
            PropEnum::FacingDirection(e) => ("eEFacingDirection", (*e).into()),
            PropEnum::FresnelTerm(e) => ("eEFresnelTerm", (*e).into()),
            PropEnum::GuiCursorSize(e) => ("eEGuiCursorSize", (*e).into()),
            PropEnum::IFOutputMode(e) => ("eEIFOutputMode", (*e).into()),
            PropEnum::IFSizeMode(e) => ("eEIFSizeMode", (*e).into()),
            PropEnum::IFTextureMode(e) => ("eEIFTextureMode", (*e).into()),
            PropEnum::ImageBlend(e) => ("eEImageBlend", (*e).into()),
            PropEnum::ImageLayerBlend(e) => ("eEImageLayerBlend", (*e).into()),
            PropEnum::LightingStyle(e) => ("eELightingStyle", (*e).into()),
            PropEnum::ListView(e) => ("eEListView", (*e).into()),
            PropEnum::ListViewAlign(e) => ("eEListViewAlign", (*e).into()),
            PropEnum::ListViewIconSize(e) => ("eEListViewIconSize", (*e).into()),
            PropEnum::ListViewItemLayout(e) => ("eEListViewItemLayout", (*e).into()),
            PropEnum::ListViewTileSize(e) => ("eEListViewTileSize", (*e).into()),
            PropEnum::LocationShape(e) => ("eELocationShape", (*e).into()),
            PropEnum::LocationTarget(e) => ("eELocationTarget", (*e).into()),
            PropEnum::MoverPlayBackMode(e) => ("eEMoverPlayBackMode", (*e).into()),
            PropEnum::OverlayMode(e) => ("eEOverlayMode", (*e).into()),
            PropEnum::PhysicRangeType(e) => ("eEPhysicRangeType", (*e).into()),
            PropEnum::PictureMode(e) => ("eEPictureMode", (*e).into()),
            PropEnum::PropertySetType(e) => ("eEPropertySetType", (*e).into()),
            PropEnum::ReflectType(e) => ("eEReflectType", (*e).into()),
            PropEnum::RigidbodyFlag(e) => ("eERigidbody_Flag", (*e).into()),
            PropEnum::RotationFrom(e) => ("eERotationFrom", (*e).into()),
            PropEnum::ShaderMaterialBRDFType(e) => ("eEShaderMaterialBRDFType", (*e).into()),
            PropEnum::ShaderMaterialBlendMode(e) => ("eEShaderMaterialBlendMode", (*e).into()),
            PropEnum::ShaderMaterialTransformation(e) => {
                ("eEShaderMaterialTransformation", (*e).into())
            }
            PropEnum::ShaderMaterialVersion(e) => ("eEShaderMaterialVersion", (*e).into()),
            PropEnum::ShadowCasterType(e) => ("eEShadowCasterType", (*e).into()),
            PropEnum::ShadowMaskIndex(e) => ("eEShadowMaskIndex", (*e).into()),
            PropEnum::ShapeAABBAdapt(e) => ("eEShapeAABBAdapt", (*e).into()),
            PropEnum::ShapeGroup(e) => ("eEShapeGroup", (*e).into()),
            PropEnum::MaterialResource2ShapeMaterial(e) => {
                ("eCMaterialResource2_eEShapeMaterial", (*e).into())
            }
            PropEnum::ShapeMaterial(e) => ("eEShapeMaterial", (*e).into()),
            PropEnum::SplitImageStyle(e) => ("eESplitImageStyle", (*e).into()),
            PropEnum::StaticIlluminated(e) => ("eEStaticIlluminated", (*e).into()),
            PropEnum::StripSpawning(e) => ("eEStripSpawning", (*e).into()),
            PropEnum::TexCoordSrcOscillatorType(e) => ("eETexCoordSrcOscillatorType", (*e).into()),
            PropEnum::TexCoordSrcRotatorType(e) => ("eETexCoordSrcRotatorType", (*e).into()),
            PropEnum::TextAlign(e) => ("eETextAlign", (*e).into()),
            PropEnum::TextureDrawStyle(e) => ("eETextureDrawStyle", (*e).into()),
            PropEnum::TicSide(e) => ("eETicSide", (*e).into()),
            PropEnum::VegetationBrushColorFunction(e) => {
                ("eEVegetationBrushColorFunction", (*e).into())
            }
            PropEnum::VegetationBrushMode(e) => ("eEVegetationBrushMode", (*e).into()),
            PropEnum::VegetationBrushPlace(e) => ("eEVegetationBrushPlace", (*e).into()),
            PropEnum::VegetationBrushProbabilityFunction(e) => {
                ("eEVegetationBrushProbabilityFunction", (*e).into())
            }
            PropEnum::VegetationBrushShape(e) => ("eEVegetationBrushShape", (*e).into()),
            PropEnum::VegetationMeshShading(e) => ("eEVegetationMeshShading", (*e).into()),
            PropEnum::VelocityDirectionFrom(e) => ("eEVelocityDirectionFrom", (*e).into()),
            PropEnum::WeatherZoneOverwrite(e) => ("eEWeatherZoneOverwrite", (*e).into()),
            PropEnum::WeatherZoneShape(e) => ("eEWeatherZoneShape", (*e).into()),
            PropEnum::AIMode(e) => ("gEAIMode", (*e).into()),
            PropEnum::AchievementViewMode(e) => ("gEAchievementViewMode", (*e).into()),
            PropEnum::AlignToTarget(e) => ("gEAlignToTarget", (*e).into()),
            PropEnum::AmbientAction(e) => ("gEAmbientAction", (*e).into()),
            PropEnum::AmountType(e) => ("gEAmountType", (*e).into()),
            PropEnum::AnchorType(e) => ("gEAnchorType", (*e).into()),
            PropEnum::AniState(e) => ("gEAniState", (*e).into()),
            PropEnum::CombatMoveStumbleAniState(e) => {
                ("gCCombatMoveStumble_gEAniState", (*e).into())
            }
            PropEnum::ArenaStatus(e) => ("gEArenaStatus", (*e).into()),
            PropEnum::Attitude(e) => ("gEAttitude", (*e).into()),
            PropEnum::BoostTarget(e) => ("gEBoostTarget", (*e).into()),
            PropEnum::BraveryOverride(e) => ("gEBraveryOverride", (*e).into()),
            PropEnum::MeleeCombatAction(e) => ("gCCombatMoveMelee_gECombatAction", (*e).into()),
            PropEnum::MoveCombatAction(e) => {
                ("gCCombatMoveScriptState_gECombatAction", (*e).into())
            }
            PropEnum::CombatAttackStumble(e) => ("gECombatAttackStumble", (*e).into()),
            PropEnum::CombatComboParade(e) => ("gECombatComboParade", (*e).into()),
            PropEnum::CombatFightAIMode(e) => ("gECombatFightAIMode", (*e).into()),
            PropEnum::CombatHitDirection(e) => ("gECombatHitDirection", (*e).into()),
            PropEnum::CombatMode(e) => ("gECombatMode", (*e).into()),
            PropEnum::MeleeCombatMove(e) => ("gCCombatMoveMelee_gECombatMove", (*e).into()),
            PropEnum::Melee2CombatMove(e) => ("gCCombatMoveMelee2_gECombatMove", (*e).into()),
            PropEnum::CombatMoveSide(e) => ("gECombatMoveSide", (*e).into()),
            PropEnum::CombatParadeType(e) => ("gECombatParadeType", (*e).into()),
            PropEnum::CombatPhaseType(e) => ("gECombatPhaseType", (*e).into()),
            PropEnum::CombatPose(e) => ("gECombatPose", (*e).into()),
            PropEnum::Comment(e) => ("gEComment", (*e).into()),
            PropEnum::CompareOperation(e) => ("gECompareOperation", (*e).into()),
            PropEnum::Crime(e) => ("gECrime", (*e).into()),
            PropEnum::DamageCalculationType(e) => ("gEDamageCalculationType", (*e).into()),
            PropEnum::DamageType(e) => ("gEDamageType", (*e).into()),
            PropEnum::DoorStatus(e) => ("gEDoorStatus", (*e).into()),
            PropEnum::EffectDecayMode(e) => ("gEEffectDecayMode", (*e).into()),
            PropEnum::EffectKillRange(e) => ("gEEffectKillRange", (*e).into()),
            PropEnum::EffectLink(e) => ("gEEffectLink", (*e).into()),
            PropEnum::EffectLoopMode(e) => ("gEEffectLoopMode", (*e).into()),
            PropEnum::EffectScriptOtherType(e) => ("gEEffectScriptOtherType", (*e).into()),
            PropEnum::EffectScriptParamType(e) => ("gEEffectScriptParamType", (*e).into()),
            PropEnum::EffectStopMode(e) => ("gEEffectStopMode", (*e).into()),
            PropEnum::EffectTargetMode(e) => ("gEEffectTargetMode", (*e).into()),
            PropEnum::EntityType(e) => ("gEEntityType", (*e).into()),
            PropEnum::Picbox2EquipSlot(e) => ("gCEquipPicbox2_gEEquipSlot", (*e).into()),
            PropEnum::EquipSlot(e) => ("gEEquipSlot", (*e).into()),
            PropEnum::Fight(e) => ("gEFight", (*e).into()),
            PropEnum::FlightPathType(e) => ("gEFlightPathType", (*e).into()),
            PropEnum::FocusNameType(e) => ("gEFocusNameType", (*e).into()),
            PropEnum::FocusPriority(e) => ("gEFocusPriority", (*e).into()),
            PropEnum::FocusSource(e) => ("gEFocusSource", (*e).into()),
            PropEnum::GUIFilterType(e) => ("gEGUIFilterType", (*e).into()),
            PropEnum::GammaRamp(e) => ("gEGammaRamp", (*e).into()),
            PropEnum::Gender(e) => ("gEGender", (*e).into()),
            PropEnum::GuardStatus(e) => ("gEGuardStatus", (*e).into()),
            PropEnum::NpcGuardStatus(e) => ("gCNPC_PS_gEGuardStatus", (*e).into()),
            PropEnum::Guild(e) => ("gEGuild", (*e).into()),
            PropEnum::NpcGuild(e) => ("gCNPC_PS_gEGuild", (*e).into()),
            PropEnum::HitDirection(e) => ("gEHitDirection", (*e).into()),
            PropEnum::HudPage(e) => ("gEHudPage", (*e).into()),
            PropEnum::HudPageProgessBar(e) => ("gCPageTimerProgressBar_gEHudPage", (*e).into()),
            PropEnum::Icon(e) => ("gEIcon", (*e).into()),
            PropEnum::InfoCondType(e) => ("gEInfoCondType", (*e).into()),
            PropEnum::QuestInfoCondType(e) => {
                ("gCInfoConditionQuestStatus_gEInfoCondType", (*e).into())
            }
            PropEnum::InfoGesture(e) => ("gEInfoGesture", (*e).into()),
            PropEnum::InfoLocation(e) => ("gEInfoLocation", (*e).into()),
            PropEnum::InfoNPCStatus(e) => ("gEInfoNPCStatus", (*e).into()),
            PropEnum::InfoNPCType(e) => ("gEInfoNPCType", (*e).into()),
            PropEnum::InfoType(e) => ("gEInfoType", (*e).into()),
            PropEnum::InfoView(e) => ("gEInfoView", (*e).into()),
            PropEnum::SkillInfoView(e) => ("gCSkillInfo_gEInfoView", (*e).into()),
            PropEnum::DialogInfoView(e) => ("gCDialogInfo_gCDialogInfo_gEInfoView", (*e).into()),
            PropEnum::HintInfoView(e) => ("gCHintStatic_gEInfoView", (*e).into()),
            PropEnum::InteractionType(e) => ("gEInteractionType", (*e).into()),
            PropEnum::SpecialInteractionUseType(e) => ("SPECIAL_gEInteractionUseType", (*e).into()),
            PropEnum::InteractionUseType(e) => ("gEInteractionUseType", (*e).into()),
            PropEnum::ItemCategory(e) => ("gEItemCategory", (*e).into()),
            PropEnum::ItemHoldType(e) => ("gEItemHoldType", (*e).into()),
            PropEnum::ItemUseType(e) => ("gEItemUseType", (*e).into()),
            PropEnum::WeaponItemUseType(e) => ("gCCombatWeaponConfig_gEItemUseType", (*e).into()),
            PropEnum::LockStatus(e) => ("gELockStatus", (*e).into()),
            PropEnum::MiscInfo(e) => ("gEMiscInfo", (*e).into()),
            PropEnum::ProgressBarMiscInfo(e) => ("gCMiscProgressBar_gEMiscInfo", (*e).into()),
            PropEnum::MouseAxis(e) => ("gEMouseAxis", (*e).into()),
            PropEnum::NavObstacleType(e) => ("gENavObstacleType", (*e).into()),
            PropEnum::NavTestResult(e) => ("gENavTestResult", (*e).into()),
            PropEnum::OtherType(e) => ("gEOtherType", (*e).into()),
            PropEnum::LootPageMode(e) => ("gCLootStatic2_gEPageMode", (*e).into()),
            PropEnum::HudPageMode(e) => ("gCHudPage2_gEPageMode", (*e).into()),
            PropEnum::PageMode(e) => ("gEPageMode", (*e).into()),
            PropEnum::PaintArea(e) => ("gEPaintArea", (*e).into()),
            PropEnum::PartyMemberType(e) => ("gEPartyMemberType", (*e).into()),
            PropEnum::QuestActor(e) => ("gEQuestActor", (*e).into()),
            PropEnum::QuestStatus(e) => ("gEQuestStatus", (*e).into()),
            PropEnum::QuestType(e) => ("gEQuestType", (*e).into()),
            PropEnum::QuickSlot(e) => ("gEQuickSlot", (*e).into()),
            PropEnum::Reason(e) => ("gEReason", (*e).into()),
            PropEnum::RecipeCategory(e) => ("gERecipeCategory", (*e).into()),
            PropEnum::ScrollStart(e) => ("gEScrollStart", (*e).into()),
            PropEnum::SessionState(e) => ("gESession_State", (*e).into()),
            PropEnum::Skill(e) => ("gESkill", (*e).into()),
            PropEnum::SkillValueBaseSkill(e) => ("gCSkillValueBase_gESkill", (*e).into()),
            PropEnum::ProgressBarSkill(e) => ("gCSkillProgressBar_gESkill", (*e).into()),
            PropEnum::SkillModifier(e) => ("gESkillModifier", (*e).into()),
            PropEnum::SpecialEntity(e) => ("gESpecialEntity", (*e).into()),
            PropEnum::Species(e) => ("gESpecies", (*e).into()),
            PropEnum::SpinButtonType(e) => ("gESpinButtonType", (*e).into()),
            PropEnum::StackType(e) => ("gEStackType", (*e).into()),
            PropEnum::StateGraphEventType(e) => ("gEStateGraphEventType", (*e).into()),
            PropEnum::ViewMode(e) => ("gEViewMode", (*e).into()),
            PropEnum::EquipViewMode(e) => ("gCEquipPicbox2_gEViewMode", (*e).into()),
            PropEnum::QuickViewMode(e) => ("gCQuickPicbox2_gEViewMode", (*e).into()),
            PropEnum::WalkMode(e) => ("gEWalkMode", (*e).into()),
            PropEnum::WrittenType(e) => ("gEWrittenType", (*e).into()),
            PropEnum::Unknown(UnknownEnum { name, val }) => (name.as_str(), *val),
        };
        write_u32(dst, discriminant)?;
        Ok(name)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PropData {
    Int(i32),
    Bool(bool),
    Short(i16),
    Float(f32),
    Long(u32),
    Char(u8),
    Vector2(Vec2),
    Vector3(Vec3),
    Vector4(Vec4),
    Quaternion(Quat),
    Matrix(glam::Mat4),
    Guid(PropertyId),
    String(String),
    ImageOrMaterialResourceString(String),
    LocString(String),
    ImageResourceString(String),
    LetterLocString(String),
    SpeedTreeResourceString(String),
    BookLocString(String),
    MeshResourceString(String),
    InfoLocString(String),
    QuestLocString(String),
    TipLocString(String),
    NPCInfoLocString(String),
    FloatColor {
        unknown: u32,
        r: f32,
        g: f32,
        b: f32,
    },
    EntityProxy(EntityProxy),
    TemplateEntityProxy(EntityProxy),
    BoundingBox(BoundingBox),
    ContainerEnum(PropEnum),
    Enum(PropEnum),
    Buffer(PropBuffer),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PropBuffer {
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(with = "crate::helpers::ser_hex")]
    pub data: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PropEnum {
    Direction(enums::Direction),
    NoiseTurbulence(enums::NoiseTurbulence),
    ImageFilterRTColorFormat(enums::ImageFilterRTColorFormat),
    ImageResource2ColorFormat(enums::ImageResource2ColorFormat),
    AnchorMode(enums::AnchorMode),
    AudioChannelFallOff(enums::AudioChannelFallOff),
    AudioChannelGroup(enums::AudioChannelGroup),
    AudioEmitterMode(enums::AudioEmitterMode),
    AudioEmitterShape(enums::AudioEmitterShape),
    BillboardTargetMode(enums::BillboardTargetMode),
    BoolOverwrite(enums::BoolOverwrite),
    RadioCheckState(enums::RadioCheckState),
    CheckBoxCheckState(enums::CheckBoxCheckState),
    CollisionGroup(enums::CollisionGroup),
    CollisionShapeType(enums::CollisionShapeType),
    ColorSrcCombinerType(enums::ColorSrcCombinerType),
    ColorSrcSampleTexRepeat(enums::ColorSrcSampleTexRepeat),
    ColorSrcSwitchRepeat(enums::ColorSrcSwitchRepeat),
    CoordinateSystem(enums::CoordinateSystem),
    DistanceType(enums::DistanceType),
    Dock(enums::Dock),
    DynamicLightEffect(enums::DynamicLightEffect),
    FacingDirection(enums::FacingDirection),
    FresnelTerm(enums::FresnelTerm),
    GuiCursorSize(enums::GuiCursorSize),
    IFOutputMode(enums::IFOutputMode),
    IFSizeMode(enums::IFSizeMode),
    IFTextureMode(enums::IFTextureMode),
    ImageBlend(enums::ImageBlend),
    ImageLayerBlend(enums::ImageLayerBlend),
    LightingStyle(enums::LightingStyle),
    ListView(enums::ListView),
    ListViewAlign(enums::ListViewAlign),
    ListViewIconSize(enums::ListViewIconSize),
    ListViewItemLayout(enums::ListViewItemLayout),
    ListViewTileSize(enums::ListViewTileSize),
    LocationShape(enums::LocationShape),
    LocationTarget(enums::LocationTarget),
    MoverPlayBackMode(enums::MoverPlayBackMode),
    OverlayMode(enums::OverlayMode),
    PhysicRangeType(enums::PhysicRangeType),
    PictureMode(enums::PictureMode),
    PropertySetType(enums::PropertySetType),
    ReflectType(enums::ReflectType),
    RigidbodyFlag(enums::RigidbodyFlag),
    RotationFrom(enums::RotationFrom),
    ShaderMaterialBRDFType(enums::ShaderMaterialBRDFType),
    ShaderMaterialBlendMode(enums::ShaderMaterialBlendMode),
    ShaderMaterialTransformation(enums::ShaderMaterialTransformation),
    ShaderMaterialVersion(enums::ShaderMaterialVersion),
    ShadowCasterType(enums::ShadowCasterType),
    ShadowMaskIndex(enums::ShadowMaskIndex),
    ShapeAABBAdapt(enums::ShapeAABBAdapt),
    ShapeGroup(enums::ShapeGroup),
    MaterialResource2ShapeMaterial(enums::MaterialResource2ShapeMaterial),
    ShapeMaterial(enums::ShapeMaterial),
    SplitImageStyle(enums::SplitImageStyle),
    StaticIlluminated(enums::StaticIlluminated),
    StripSpawning(enums::StripSpawning),
    TexCoordSrcOscillatorType(enums::TexCoordSrcOscillatorType),
    TexCoordSrcRotatorType(enums::TexCoordSrcRotatorType),
    TextAlign(enums::TextAlign),
    TextureDrawStyle(enums::TextureDrawStyle),
    TicSide(enums::TicSide),
    VegetationBrushColorFunction(enums::VegetationBrushColorFunction),
    VegetationBrushMode(enums::VegetationBrushMode),
    VegetationBrushPlace(enums::VegetationBrushPlace),
    VegetationBrushProbabilityFunction(enums::VegetationBrushProbabilityFunction),
    VegetationBrushShape(enums::VegetationBrushShape),
    VegetationMeshShading(enums::VegetationMeshShading),
    VelocityDirectionFrom(enums::VelocityDirectionFrom),
    WeatherZoneOverwrite(enums::WeatherZoneOverwrite),
    WeatherZoneShape(enums::WeatherZoneShape),
    AIMode(enums::AIMode),
    AchievementViewMode(enums::AchievementViewMode),
    AlignToTarget(enums::AlignToTarget),
    AmbientAction(enums::AmbientAction),
    AmountType(enums::AmountType),
    AnchorType(enums::AnchorType),
    AniState(enums::AniState),
    CombatMoveStumbleAniState(enums::CombatMoveStumbleAniState),
    ArenaStatus(enums::ArenaStatus),
    Attitude(enums::Attitude),
    BoostTarget(enums::BoostTarget),
    BraveryOverride(enums::BraveryOverride),
    MeleeCombatAction(enums::MeleeCombatAction),
    MoveCombatAction(enums::MoveCombatAction),
    CombatAttackStumble(enums::CombatAttackStumble),
    CombatComboParade(enums::CombatComboParade),
    CombatFightAIMode(enums::CombatFightAIMode),
    CombatHitDirection(enums::CombatHitDirection),
    CombatMode(enums::CombatMode),
    MeleeCombatMove(enums::MeleeCombatMove),
    Melee2CombatMove(enums::Melee2CombatMove),
    CombatMoveSide(enums::CombatMoveSide),
    CombatParadeType(enums::CombatParadeType),
    CombatPhaseType(enums::CombatPhaseType),
    CombatPose(enums::CombatPose),
    Comment(enums::Comment),
    CompareOperation(enums::CompareOperation),
    Crime(enums::Crime),
    DamageCalculationType(enums::DamageCalculationType),
    DamageType(enums::DamageType),
    DoorStatus(enums::DoorStatus),
    EffectDecayMode(enums::EffectDecayMode),
    EffectKillRange(enums::EffectKillRange),
    EffectLink(enums::EffectLink),
    EffectLoopMode(enums::EffectLoopMode),
    EffectScriptOtherType(enums::EffectScriptOtherType),
    EffectScriptParamType(enums::EffectScriptParamType),
    EffectStopMode(enums::EffectStopMode),
    EffectTargetMode(enums::EffectTargetMode),
    EntityType(enums::EntityType),
    Picbox2EquipSlot(enums::Picbox2EquipSlot),
    EquipSlot(enums::EquipSlot),
    Fight(enums::Fight),
    FlightPathType(enums::FlightPathType),
    FocusNameType(enums::FocusNameType),
    FocusPriority(enums::FocusPriority),
    FocusSource(enums::FocusSource),
    GUIFilterType(enums::GUIFilterType),
    GammaRamp(enums::GammaRamp),
    Gender(enums::Gender),
    GuardStatus(enums::GuardStatus),
    NpcGuardStatus(enums::NpcGuardStatus),
    Guild(enums::Guild),
    NpcGuild(enums::NpcGuild),
    HitDirection(enums::HitDirection),
    HudPage(enums::HudPage),
    HudPageProgessBar(enums::HudPageProgessBar),
    Icon(enums::Icon),
    InfoCondType(enums::InfoCondType),
    QuestInfoCondType(enums::QuestInfoCondType),
    InfoGesture(enums::InfoGesture),
    InfoLocation(enums::InfoLocation),
    InfoNPCStatus(enums::InfoNPCStatus),
    InfoNPCType(enums::InfoNPCType),
    InfoType(enums::InfoType),
    InfoView(enums::InfoView),
    SkillInfoView(enums::SkillInfoView),
    DialogInfoView(enums::DialogInfoView),
    HintInfoView(enums::HintInfoView),
    InteractionType(enums::InteractionType),
    SpecialInteractionUseType(enums::SpecialInteractionUseType),
    InteractionUseType(enums::InteractionUseType),
    ItemCategory(enums::ItemCategory),
    ItemHoldType(enums::ItemHoldType),
    ItemUseType(enums::ItemUseType),
    WeaponItemUseType(enums::WeaponItemUseType),
    LockStatus(enums::LockStatus),
    MiscInfo(enums::MiscInfo),
    ProgressBarMiscInfo(enums::ProgressBarMiscInfo),
    MouseAxis(enums::MouseAxis),
    NavObstacleType(enums::NavObstacleType),
    NavTestResult(enums::NavTestResult),
    OtherType(enums::OtherType),
    LootPageMode(enums::LootPageMode),
    HudPageMode(enums::HudPageMode),
    PageMode(enums::PageMode),
    PaintArea(enums::PaintArea),
    PartyMemberType(enums::PartyMemberType),
    QuestActor(enums::QuestActor),
    QuestStatus(enums::QuestStatus),
    QuestType(enums::QuestType),
    QuickSlot(enums::QuickSlot),
    Reason(enums::Reason),
    RecipeCategory(enums::RecipeCategory),
    ScrollStart(enums::ScrollStart),
    SessionState(enums::SessionState),
    Skill(enums::Skill),
    SkillValueBaseSkill(enums::SkillValueBaseSkill),
    ProgressBarSkill(enums::ProgressBarSkill),
    SkillModifier(enums::SkillModifier),
    SpecialEntity(enums::SpecialEntity),
    Species(enums::Species),
    SpinButtonType(enums::SpinButtonType),
    StackType(enums::StackType),
    StateGraphEventType(enums::StateGraphEventType),
    ViewMode(enums::ViewMode),
    EquipViewMode(enums::EquipViewMode),
    QuickViewMode(enums::QuickViewMode),
    WalkMode(enums::WalkMode),
    WrittenType(enums::WrittenType),
    Unknown(UnknownEnum),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UnknownEnum {
    pub name: String,
    pub val: u32,
}
