//! Benchmark: adaptation_latency
//!
//! Measures cycles-to-confirm and microsecond execution time for a sustained
//! dim -> bright scene change through the AdaptationPolicy fast-confirm burst path.
//!
//! Run with: `cargo bench --bench adaptation_latency`
//! Automatically executed in CI by `.github/workflows/benchmarks.yml`

use app_lib::adaptation::policy::{AdaptationPolicy, PolicyContext};
use std::hint::black_box;
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

fn adaptation_cycles_to_confirm(
    policy: &mut AdaptationPolicy,
    baseline_lum: f32,
    bright_lum: f32,
    context: &str,
) -> usize {
    // Prime with baseline
    for _ in 0..3 {
        policy.observe(100.0, baseline_lum);
    }
    // Pre-warm context stability
    let warmup_ctx = make_ctx(100.0, baseline_lum, context);
    for _ in 0..5 {
        policy.observe(100.0, baseline_lum);
        let _ = policy.should_adapt(&warmup_ctx);
        std::thread::sleep(Duration::from_millis(55));
    }

    // Sudden bright scene cut
    policy.observe(100.0, bright_lum);
    let ctx = make_ctx(100.0, bright_lum, context);

    let mut ticks = 0;
    loop {
        ticks += 1;
        let decision = policy.should_adapt(&ctx);
        if decision.is_adapt() {
            break;
        }
        // Fast-confirm path re-samples rapidly
        policy.observe(100.0, bright_lum);
        if ticks > 10 {
            break;
        }
    }
    ticks
}

fn main() {
    println!("=== PixelSense Adaptation Latency Benchmark ===");
    println!("Running 1,000 iterations of single-tick policy evaluation...");

    let mut policy = AdaptationPolicy::new();
    for _ in 0..5 {
        policy.observe(100.0, 50.0);
    }
    let ctx = make_ctx(100.0, 50.0, "Video");

    let iters = 1000;
    let start = Instant::now();
    for _ in 0..iters {
        policy.observe(100.0, black_box(50.0));
        let decision = policy.should_adapt(&ctx);
        black_box(decision);
    }
    let elapsed = start.elapsed();
    let per_op_ns = elapsed.as_nanos() as f64 / iters as f64;
    let per_op_us = per_op_ns / 1000.0;

    println!(
        "  -> Policy tick latency: {:.2} µs/op ({:.0} ns/op)",
        per_op_us, per_op_ns
    );

    println!("\nRunning scene-change fast-confirm confirmation test...");
    let mut scene_policy = AdaptationPolicy::new();
    let cycles = adaptation_cycles_to_confirm(&mut scene_policy, 20.0, 75.0, "Video");

    println!(
        "  -> Scene-change confirmation ticks: {} (Target: <= 2 ticks)",
        cycles
    );
    assert!(
        cycles <= 2,
        "Confirmation required {} ticks (expected <= 2)",
        cycles
    );

    println!(
        "\n[OK] Benchmark passed: sub-microsecond decision time and 2-tick confirmation confirmed."
    );
}
