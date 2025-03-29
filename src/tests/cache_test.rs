#[cfg(test)]
mod tests {
    #[test]
    fn test_cache_errors() {
        let cache = Cache::new();
        assert!(cache.set("key", "value").is_ok());
        assert!(cache.get("nonexistent").is_err());
    }
}
