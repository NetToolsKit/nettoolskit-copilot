use nettoolskit_templating::TemplateEngine;
use serde_json::json;

#[tokio::test]
async fn test_template_caching() {
    // Arrange
    let engine = TemplateEngine::new();
    let data = json!({"name": "Test"});

    // Act - First render should populate the cache.
    let result1 = engine
        .render_from_string("Hello {{name}}!", &data, "cached_test".to_string())
        .await;
    let (cache_size_after_first_render, _) = engine.cache_stats();

    // Act - Second render should reuse the existing compiled template.
    let result2 = engine
        .render_from_string("Hello {{name}}!", &data, "cached_test".to_string())
        .await;
    let (cache_size_after_second_render, _) = engine.cache_stats();

    // Assert
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert_eq!(result1.unwrap(), result2.unwrap());
    assert_eq!(
        cache_size_after_first_render, 1,
        "First render should populate exactly one cached template"
    );
    assert_eq!(
        cache_size_after_second_render, 1,
        "Second render should reuse the cached template instead of growing the cache"
    );
}

#[tokio::test]
async fn test_cache_stats_accuracy() {
    let engine = TemplateEngine::new();
    let data = json!({"name": "Test"});

    // Initial state
    let (size, hits) = engine.cache_stats();
    assert_eq!(size, 0);
    assert_eq!(hits, 0);

    // Render first template
    let _ = engine
        .render_from_string("Template 1: {{name}}", &data, "template1".to_string())
        .await;

    let (size, _) = engine.cache_stats();
    assert_eq!(size, 1);

    // Render second template
    let _ = engine
        .render_from_string("Template 2: {{name}}", &data, "template2".to_string())
        .await;

    let (size, _) = engine.cache_stats();
    assert_eq!(size, 2);
}