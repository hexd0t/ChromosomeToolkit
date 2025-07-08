use std::{env, path::PathBuf};

use gluegen::generate_glue;

fn main() {
    // rerun build script if definition file is changed:
    let lib_name = "Engine";
    println!("cargo::rerun-if-changed=lib/{lib_name}.def");
    println!("cargo::rustc-link-lib={lib_name}");
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut lib_dir = manifest_dir.clone();
    lib_dir.push("lib");
    println!("cargo:rustc-link-search=all={}", lib_dir.display());

    let def_file = std::fs::File::open(format!("lib/{lib_name}.def")).unwrap();
    let manual_types = &[];
    let additional_opaque_types = &[
        "eSCullPlaneBoxHelper",
        "eCGfxShared_eEPrimitiveType",
        "eCGfxShared_eEGraphicMemoryType",
        "eCGfxShared_eSGfxGaugeValues",
        "eCGizmoTransform_eCIteratorPrimitiveRotateBox",
        "eCGizmoTransformCapsule_eCIteratorPrimitiveBottomArea",
        "eCGizmoTransformCapsule_eCIteratorPrimitiveCapsuleCap",
        "eCGizmoTransformCapsule_eCIteratorPrimitiveCapsuleCylinder",
        "eCGizmoTransformCapsule_eCIteratorPrimitiveRadiusArea",
        "eCGizmoTransformCapsule_eCIteratorPrimitiveTopArea",
        "eCGizmoTransformOBB_eCIteratorPrimitiveBox",
        "eCGizmoTransformOBB_eCIteratorPrimitiveBackArea",
        "eCGizmoTransformOBB_eCIteratorPrimitiveFrontArea",
        "eCGizmoTransformOBB_eCIteratorPrimitiveLeftArea",
        "eCGizmoTransformOBB_eCIteratorPrimitiveRightArea",
        "eCGizmoTransformOBB_eCIteratorPrimitiveTopArea",
        "eCGizmoTransformOBB_eCIteratorPrimitiveBottomArea",
        "eCGizmoTransformSphere_eCIteratorPrimitiveRadiusBackArea",
        "eCGizmoTransformSphere_eCIteratorPrimitiveRadiusBottomArea",
        "eCGizmoTransformSphere_eCIteratorPrimitiveRadiusFrontArea",
        "eCGizmoTransformSphere_eCIteratorPrimitiveRadiusLeftArea",
        "eCGizmoTransformSphere_eCIteratorPrimitiveRadiusRightArea",
        "eCGizmoTransformSphere_eCIteratorPrimitiveRadiusTopArea",
        "eCGizmoTransformSphere_eCIteratorPrimitiveSphere",
        "eCGizmoTranslateAndScaleBox_eCIteratorPrimitiveFrontArea",
        "eCGizmoTranslateAndScaleBox_eCIteratorPrimitiveLeftArea",
        "eCGizmoTranslateAndScaleBox_eCIteratorPrimitiveRightArea",
        "eCGizmoTranslateAndScaleBox_eCIteratorPrimitiveTopArea",
        "eCGizmoTranslateAndScaleBox_eCIteratorPrimitiveBackArea",
        "eCGizmoTranslateAndScaleBox_eCIteratorPrimitiveBottomArea",
        "eCGizmoTranslateAndScaleBox_eCIteratorPrimitiveBox",
        "eCGizmoTranslateAndScaleSphere_eCIteratorPrimitiveRadiusBackArea",
        "eCGizmoTranslateAndScaleSphere_eCIteratorPrimitiveRadiusBottomArea",
        "eCGizmoTranslateAndScaleSphere_eCIteratorPrimitiveRadiusFrontArea",
        "eCGizmoTranslateAndScaleSphere_eCIteratorPrimitiveRadiusLeftArea",
        "eCGizmoTranslateAndScaleSphere_eCIteratorPrimitiveRadiusRightArea",
        "eCGizmoTranslateAndScaleSphere_eCIteratorPrimitiveRadiusTopArea",
        "eCGizmoTranslateAndScaleSphere_eCIteratorPrimitiveSphere",
        "eCImageResource2_EQuality",
        "eCImageResource2_SFolderQuality",
        "eCPrimitiveWeatherZoneBox",
        "eCPrimitiveWeatherZoneRadius",
        "eCPrimitiveWeatherZoneRect",
        "eCPrimitivePointLightRadius",
        "eCPrimitivePointLightCenter",
        "eCShaderConstantsBase",
        "eCShaderDefault_eCGlobalShaderConstantsDefault",
        "eCShaderLightStreaks_eCGlobalShaderConstantsLightStreaks",
        "eSShaderMaterialPass",
        "eCShaderParticle_eCGlobalShaderConstantsParticle",
        "eCGizmoTranslateAndScale_eCIteratorPrimitiveMoveEdge",
        "eCGizmoTranslateAndScale_eCIteratorPrimitiveMoveArrow",
        "eCGizmoTranslateAndScale_eCIteratorPrimitiveMoveLine",
        "eCGizmoTranslateAndScale_eCIteratorPrimitiveScale",
        "eCVideoPlayer",
        "eSRayIntersectionDesc",
        "eCRenderOp2_eSSortID",
        "eCSubMesh_eSBuildQuadTreeNode",
        "eSShaderMaterialIllumination",
        "EMotionFX_ActorInstance",
    ];
    generate_glue(def_file, lib_name, manual_types, additional_opaque_types);
}
