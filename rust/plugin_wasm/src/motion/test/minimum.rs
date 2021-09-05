/*
  Copyright (c) 2015-2021 hkrn All rights reserved

  This file is part of emapp component and it's licensed under Mozilla Public License. see LICENSE.md for more details.
*/

use anyhow::Result;
use maplit::hashmap;
use serde_json::json;
use wasmer_wasi::WasiEnv;

use crate::motion::{
    plugin::MotionIOPluginController,
    test::{
        create_random_data, create_wasi_env, flush_plugin_output, read_plugin_output, PluginOutput,
    },
};

use super::inner_create_controller;

fn create_controller(env: &mut WasiEnv) -> Result<MotionIOPluginController> {
    inner_create_controller("plugin_wasm_test_motion_minimum/target/wasm32-wasi/release/deps/plugin_wasm_test_motion_minimum.wasm", env)
}

#[test]
fn create() -> Result<()> {
    let mut env = create_wasi_env()?;
    let result = create_controller(&mut env);
    assert!(result.is_ok());
    let mut controller = result?;
    assert!(controller.initialize().is_ok());
    assert!(controller.create().is_ok());
    assert_eq!(
        vec![
            PluginOutput {
                function: "nanoemApplicationPluginMotionIOCreate".to_owned(),
                ..Default::default()
            },
            PluginOutput {
                function: "nanoemApplicationPluginMotionIOCountAllFunctions".to_owned(),
                ..Default::default()
            },
            PluginOutput {
                function: "nanoemApplicationPluginMotionIOGetName".to_owned(),
                ..Default::default()
            },
            PluginOutput {
                function: "nanoemApplicationPluginMotionIOGetFunctionName".to_owned(),
                arguments: Some(hashmap! {
                    "index".to_owned() => json!(0),
                })
            },
            PluginOutput {
                function: "nanoemApplicationPluginMotionIOGetVersion".to_owned(),
                ..Default::default()
            }
        ],
        read_plugin_output(&mut env)?
    );
    assert_eq!(1, controller.count_all_functions());
    assert_eq!(
        "plugin_wasm_test_motion_full: function0 (1.2.3)",
        controller.function_name(0)?.to_str()?
    );
    controller.destroy();
    controller.terminate();
    assert_eq!(
        vec![PluginOutput {
            function: "nanoemApplicationPluginMotionIODestroy".to_owned(),
            ..Default::default()
        },],
        read_plugin_output(&mut env)?
    );
    Ok(())
}

#[test]
fn set_language() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    assert!(controller.set_language(0).is_ok());
    assert_eq!(
        vec![PluginOutput {
            function: "nanoemApplicationPluginMotionIOSetLanguage".to_owned(),
            arguments: Some(hashmap! {
                "value".to_owned() => json!(0),
            })
        },],
        read_plugin_output(&mut env)?
    );
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn execute() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    let data = create_random_data(4096);
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    assert!(controller.set_input_model_data(&data).is_ok());
    assert!(controller.set_input_motion_data(&data).is_ok());
    assert!(controller.execute().is_ok());
    let output = controller.get_output_data();
    assert!(output.is_ok());
    let output_motion_data = output?;
    assert_eq!(
        vec![
            PluginOutput {
                function: "nanoemApplicationPluginMotionIOSetInputMotionData".to_owned(),
                arguments: Some(hashmap! {
                    "data".to_owned() => json!(data),
                    "length".to_owned() => json!(data.len()),
                    "status".to_owned() => json!(0),
                })
            },
            PluginOutput {
                function: "nanoemApplicationPluginMotionIOExecute".to_owned(),
                arguments: Some(hashmap! {
                    "status".to_owned() => json!(0),
                })
            },
            PluginOutput {
                function: "nanoemApplicationPluginMotionIOGetOutputMotionDataSize".to_owned(),
                arguments: Some(hashmap! {
                    "length".to_owned() => json!(output_motion_data.len()),
                })
            },
            PluginOutput {
                function: "nanoemApplicationPluginMotionIOGetOutputMotionData".to_owned(),
                arguments: Some(hashmap! {
                    "data".to_owned() => json!(output_motion_data),
                    "length".to_owned() => json!(output_motion_data.len()),
                    "status".to_owned() => json!(0),
                })
            },
        ],
        read_plugin_output(&mut env)?
    );
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn set_all_selected_accessory_keyframes() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    let data = &[1, 4, 9, 16, 25, u32::MAX];
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    assert!(controller
        .set_all_selected_accessory_keyframes(data)
        .is_ok());
    assert!(read_plugin_output(&mut env)?.is_empty());
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn set_all_selected_camera_keyframes() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    let data = &[1, 4, 9, 16, 25, u32::MAX];
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    assert!(controller.set_all_selected_camera_keyframes(data).is_ok());
    assert!(read_plugin_output(&mut env)?.is_empty());
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn set_all_selected_light_keyframes() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    let data = &[1, 4, 9, 16, 25, u32::MAX];
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    assert!(controller.set_all_selected_light_keyframes(data).is_ok());
    assert!(read_plugin_output(&mut env)?.is_empty());
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn set_all_selected_model_keyframes() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    let data = &[1, 4, 9, 16, 25, u32::MAX];
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    assert!(controller.set_all_selected_model_keyframes(data).is_ok());
    assert!(read_plugin_output(&mut env)?.is_empty());
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn set_all_selected_self_shadow_keyframes() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    let data = &[1, 4, 9, 16, 25, u32::MAX];
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    assert!(controller
        .set_all_selected_self_shadow_keyframes(data)
        .is_ok());
    assert!(read_plugin_output(&mut env)?.is_empty());
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn set_all_named_selected_bone_keyframes() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    let data = &[1, 4, 9, 16, 25, u32::MAX];
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    assert!(controller
        .set_all_named_selected_bone_keyframes("bone", data)
        .is_ok());
    assert!(read_plugin_output(&mut env)?.is_empty());
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn set_all_named_selected_morph_keyframes() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    let data = &[1, 4, 9, 16, 25, u32::MAX];
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    assert!(controller
        .set_all_named_selected_morph_keyframes("morph", data)
        .is_ok());
    assert!(read_plugin_output(&mut env)?.is_empty());
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn set_audio_description() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    let data = create_random_data(4096);
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    assert!(controller.set_audio_description(&data).is_ok());
    assert!(read_plugin_output(&mut env)?.is_empty());
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn set_audio_data() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    let data = create_random_data(4096);
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    assert!(controller.set_audio_data(&data).is_ok());
    assert!(read_plugin_output(&mut env)?.is_empty());
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn set_camera_description() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    let data = create_random_data(4096);
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    assert!(controller.set_camera_description(&data).is_ok());
    assert!(read_plugin_output(&mut env)?.is_empty());
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn set_light_description() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    let data = create_random_data(4096);
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    assert!(controller.set_light_description(&data).is_ok());
    assert!(read_plugin_output(&mut env)?.is_empty());
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn ui_window() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    let data = create_random_data(4096);
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    assert!(controller.load_ui_window_layout().is_ok());
    let output = controller.get_ui_window_layout();
    assert!(output.is_ok());
    assert!(output?.is_empty());
    let mut reload = false;
    assert!(controller
        .set_ui_component_layout("ui_window", &data, &mut reload)
        .is_ok());
    assert!(read_plugin_output(&mut env)?.is_empty());
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn get_failure_reason() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    let result = controller.failure_reason();
    assert!(result.is_ok());
    assert_eq!("Failure Reason", result?);
    assert_eq!(
        vec![PluginOutput {
            function: "nanoemApplicationPluginMotionIOGetFailureReason".to_owned(),
            ..Default::default()
        },],
        read_plugin_output(&mut env)?
    );
    controller.destroy();
    controller.terminate();
    Ok(())
}

#[test]
fn get_recovery_suggestion() -> Result<()> {
    let mut env = create_wasi_env()?;
    let mut controller = create_controller(&mut env)?;
    controller.initialize()?;
    controller.create()?;
    controller.set_function(0)?;
    flush_plugin_output(&mut env)?;
    let result = controller.recovery_suggestion();
    assert!(result.is_ok());
    assert_eq!("Recovery Suggestion", result?);
    assert_eq!(
        vec![PluginOutput {
            function: "nanoemApplicationPluginMotionIOGetRecoverySuggestion".to_owned(),
            ..Default::default()
        },],
        read_plugin_output(&mut env)?
    );
    controller.destroy();
    controller.terminate();
    Ok(())
}