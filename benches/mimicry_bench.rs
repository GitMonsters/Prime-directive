use consciousness_experiments::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_system1_cache_lookup(c: &mut Criterion) {
    let store = AiProfileStore::default();
    let _profile = store.get("gpt4o").unwrap().clone();
    let mut cache = SignatureCache::new();
    // Pre-warm the cache
    let mut analyzer = BehaviorAnalyzer::new();
    let responses =
        vec!["I'd be happy to help with that. Here's what you need to know...".to_string()];
    let sig = analyzer.build_signature("gpt4o", &responses);
    cache.compile_from(&sig);

    c.bench_function("system1_cache_lookup", |b| {
        b.iter(|| {
            let result = cache.lookup("gpt4o");
            black_box(result.is_some());
        })
    });
}

fn bench_system2_analyze_response(c: &mut Criterion) {
    let analyzer = BehaviorAnalyzer::new();
    let sample = "I'd be happy to help you with that! Let me break this down step by step. First, we need to consider the key factors involved. It's worth noting that there are multiple approaches, but I think the most effective one would be...";

    c.bench_function("system2_analyze_response", |b| {
        b.iter(|| {
            black_box(analyzer.analyze_response(black_box(sample)));
        })
    });
}

fn bench_profile_similarity(c: &mut Criterion) {
    let store = AiProfileStore::default();
    let gpt4o = store.get("gpt4o").unwrap();
    let claude = store.get("claude").unwrap();

    c.bench_function("profile_similarity", |b| {
        b.iter(|| {
            black_box(gpt4o.similarity_to(black_box(claude)));
        })
    });
}

fn bench_template_generation(c: &mut Criterion) {
    let store = AiProfileStore::default();
    let profile = store.get("claude").unwrap().clone();
    let mut template_store = TemplateStore::new();
    let lib = template_store.get_or_create(&profile);

    c.bench_function("template_generation", |b| {
        b.iter(|| {
            black_box(lib.generate(
                black_box("How do neural networks work?"),
                black_box(&profile.response_style),
            ));
        })
    });
}

fn bench_persona_snapshot_roundtrip(c: &mut Criterion) {
    let store = AiProfileStore::default();
    let profile = store.get("gpt4o").unwrap();
    let persona = CompoundPersona::from_profile(profile);
    let snapshot = persona.snapshot();
    let json = serde_json::to_string(&snapshot).unwrap();

    c.bench_function("persona_snapshot_serialize", |b| {
        b.iter(|| {
            black_box(serde_json::to_string(black_box(&snapshot)).unwrap());
        })
    });

    c.bench_function("persona_snapshot_deserialize", |b| {
        b.iter(|| {
            black_box(serde_json::from_str::<CompoundPersonaSnapshot>(black_box(&json)).unwrap());
        })
    });
}

fn bench_system1_vs_system2(c: &mut Criterion) {
    let mut group = c.benchmark_group("system1_vs_system2");

    // Setup System 1
    let store = AiProfileStore::default();
    let _profile = store.get("gpt4o").unwrap().clone();
    let mut cache = SignatureCache::new();
    let mut analyzer = BehaviorAnalyzer::new();
    let responses = vec!["Test response for caching".to_string()];
    let sig = analyzer.build_signature("gpt4o", &responses);
    cache.compile_from(&sig);

    let sample = "I'd be happy to help! Let me explain this concept clearly...";

    group.bench_function("system1_cached_lookup", |b| {
        b.iter(|| {
            let result = cache.lookup("gpt4o");
            black_box(result.is_some());
        })
    });

    group.bench_function("system2_full_analysis", |b| {
        b.iter(|| {
            let mut a = BehaviorAnalyzer::new();
            let responses = vec![black_box(sample).to_string()];
            let sig = a.build_signature("gpt4o", &responses);
            black_box(sig);
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_system1_cache_lookup,
    bench_system2_analyze_response,
    bench_profile_similarity,
    bench_template_generation,
    bench_persona_snapshot_roundtrip,
    bench_system1_vs_system2,
);
criterion_main!(benches);
