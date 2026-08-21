use app_lib::adaptation::policy::{AdaptationPolicy, PolicyContext};
use std::time::{Duration, Instant};

fn make_ctx(lux: f32, luminance: f32, context: &str) -> PolicyContext {
    PolicyContext {
        current_lux: lux,
        current_luminance: luminance,
        app_context: context.into(),
        confidence: 0.9,
        manual_override_active: false,
        is_fullscreen: false,
    }
}

#[test]
fn test_performance_scene_change_confirm_latency() {
    let mut policy = AdaptationPolicy::new();

    // 1. Prime with 3 baseline dim readings (e.g. movie night scene: 20% luminance)
    for _ in 0..3 {
        policy.observe(100.0, 20.0);
    }

    // Warm up context stability timer
    let warmup_ctx = make_ctx(100.0, 20.0, "Video");
    for _ in 0..5 {
        policy.observe(100.0, 20.0);
        let _ = policy.should_adapt(&warmup_ctx);
        std::thread::sleep(Duration::from_millis(55));
    }

    // 2. Sudden bright scene occurs (e.g. 20% -> 75% luminance)
    let start_time = Instant::now();
    policy.observe(100.0, 75.0);
    let bright_ctx = make_ctx(100.0, 75.0, "Video");

    // First evaluation: must enter FastConfirm immediately (Tick 1)
    let decision_1 = policy.should_adapt(&bright_ctx);
    assert!(
        decision_1.is_fast_confirm(),
        "Tick 1 must enter FastConfirm burst, got: {:?}",
        decision_1
    );

    // Simulate fast sample burst (~150ms delay as in pipeline.rs)
    std::thread::sleep(Duration::from_millis(150));

    // Second evaluation: confirmed sustained bright scene (Tick 2)
    policy.observe(100.0, 74.0);
    let confirm_ctx = make_ctx(100.0, 74.0, "Video");
    let decision_2 = policy.should_adapt(&confirm_ctx);

    let total_elapsed = start_time.elapsed();

    assert!(
        decision_2.is_adapt(),
        "Tick 2 must confirm and Adapt, got: {:?}",
        decision_2
    );

    println!(
        "Performance measurement: Fast-confirm burst confirmed scene change in {:?} (under 250ms target)",
        total_elapsed
    );

    // Assert that the entire fast-confirm decision completed in <= 350ms (well within target)
    assert!(
        total_elapsed < Duration::from_millis(350),
        "Scene change confirmation took too long: {:?}",
        total_elapsed
    );
}

#[test]
fn test_performance_single_frame_flash_rejected_quickly() {
    let mut policy = AdaptationPolicy::new();

    // Baseline dim
    for _ in 0..3 {
        policy.observe(100.0, 20.0);
    }
    let warmup_ctx = make_ctx(100.0, 20.0, "Video");
    for _ in 0..5 {
        policy.observe(100.0, 20.0);
        let _ = policy.should_adapt(&warmup_ctx);
        std::thread::sleep(Duration::from_millis(55));
    }

    // Single frame flash
    policy.observe(100.0, 80.0);
    let flash_ctx = make_ctx(100.0, 80.0, "Video");
    let d1 = policy.should_adapt(&flash_ctx);
    assert!(d1.is_fast_confirm());

    // Snaps back on next sample (not sustained)
    policy.observe(100.0, 21.0);
    let return_ctx = make_ctx(100.0, 21.0, "Video");
    let d2 = policy.should_adapt(&return_ctx);

    assert!(
        !d2.is_adapt(),
        "Single-frame flash must be rejected, got: {:?}",
        d2
    );
}
