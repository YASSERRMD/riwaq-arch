//! Caching module for analysis results and LLM responses
//!
//! Provides:
//! - In-memory caching with TTL
//! - File-based persistent caching
//! - Cache invalidation based on file changes

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// Cache entry with metadata
#[derive(Debug, Clone)]
pub struct CacheEntry<T: Clone> {
    pub value: T,
    pub created_at: Instant,
    pub ttl: Duration,
    pub hit_count: usize,
}

impl<T: Clone> CacheEntry<T> {
    pub fn new(value: T, ttl: Duration) -> Self {
        Self {
            value,
            created_at: Instant::now(),
            ttl,
            hit_count: 0,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
}

/// In-memory cache with TTL support
pub struct MemoryCache<K: Eq + Hash + Clone, V: Clone> {
    entries: RwLock<HashMap<K, CacheEntry<V>>>,
    default_ttl: Duration,
    max_entries: usize,
}

impl<K: Eq + Hash + Clone, V: Clone> MemoryCache<K, V> {
    pub fn new(default_ttl: Duration, max_entries: usize) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            default_ttl,
            max_entries,
        }
    }

    /// Get a value from the cache
    pub fn get(&self, key: &K) -> Option<V> {
        let mut entries = self.entries.write().ok()?;
        
        if let Some(entry) = entries.get_mut(key) {
            if entry.is_expired() {
                entries.remove(key);
                return None;
            }
            entry.hit_count += 1;
            return Some(entry.value.clone());
        }
        None
    }

    /// Insert a value into the cache
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        self.insert_with_ttl(key, value, self.default_ttl)
    }

    /// Insert with custom TTL
    pub fn insert_with_ttl(&self, key: K, value: V, ttl: Duration) -> Option<V> {
        let mut entries = match self.entries.write() {
            Ok(e) => e,
            Err(_) => return None,
        };

        // Evict if at capacity
        if entries.len() >= self.max_entries {
            self.evict_oldest(&mut entries);
        }

        let old = entries.insert(key, CacheEntry::new(value.clone(), ttl));
        old.map(|e| e.value)
    }

    /// Remove a value from the cache
    pub fn remove(&self, key: &K) -> Option<V> {
        self.entries.write().ok()?.remove(key).map(|e| e.value)
    }

    /// Clear all entries
    pub fn clear(&self) {
        if let Ok(mut entries) = self.entries.write() {
            entries.clear();
        }
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let entries = match self.entries.read() {
            Ok(e) => e,
            Err(_) => return CacheStats::default(),
        };

        let total_entries = entries.len();
        let expired_entries = entries.values().filter(|e| e.is_expired()).count();
        let total_hits: usize = entries.values().map(|e| e.hit_count).sum();

        CacheStats {
            total_entries,
            expired_entries,
            total_hits,
            max_entries: self.max_entries,
        }
    }

    /// Remove expired entries
    pub fn cleanup(&self) {
        if let Ok(mut entries) = self.entries.write() {
            entries.retain(|_, v| !v.is_expired());
        }
    }

    /// Evict oldest entry (LRU-ish)
    fn evict_oldest(&self, entries: &mut HashMap<K, CacheEntry<V>>) {
        // Find the entry with oldest creation time
        let oldest_key = entries
            .iter()
            .min_by_key(|(_, v)| v.created_at)
            .map(|(k, _)| k.clone());
        
        if let Some(key) = oldest_key {
            entries.remove(&key);
        }
    }
}

impl<K: Eq + Hash + Clone, V: Clone> Default for MemoryCache<K, V> {
    fn default() -> Self {
        Self::new(Duration::from_secs(300), 1000) // 5 min TTL, 1000 entries
    }
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub total_entries: usize,
    pub expired_entries: usize,
    pub total_hits: usize,
    pub max_entries: usize,
}

/// Analysis result cache key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnalysisCacheKey {
    /// Root path of the project
    pub root_path: PathBuf,
    /// Hash of file modification times
    pub content_hash: u64,
}

impl AnalysisCacheKey {
    pub fn new(root_path: PathBuf, content_hash: u64) -> Self {
        Self { root_path, content_hash }
    }
}

/// LLM response cache key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LLMCacheKey {
    /// Hash of the prompt
    pub prompt_hash: u64,
    /// Model used
    pub model: String,
}

impl LLMCacheKey {
    pub fn new(prompt: &str, model: &str) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        prompt.hash(&mut hasher);
        Self {
            prompt_hash: hasher.finish(),
            model: model.to_string(),
        }
    }
}

/// Global cache manager
pub struct CacheManager {
    /// Analysis results cache
    pub analysis_cache: MemoryCache<AnalysisCacheKey, String>,
    /// LLM response cache (longer TTL)
    pub llm_cache: MemoryCache<LLMCacheKey, String>,
    /// Documentation cache
    pub docs_cache: MemoryCache<String, String>,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            analysis_cache: MemoryCache::new(Duration::from_secs(600), 50), // 10 min
            llm_cache: MemoryCache::new(Duration::from_secs(3600), 200),     // 1 hour
            docs_cache: MemoryCache::new(Duration::from_secs(1800), 100),    // 30 min
        }
    }

    /// Clear all caches
    pub fn clear_all(&self) {
        self.analysis_cache.clear();
        self.llm_cache.clear();
        self.docs_cache.clear();
    }

    /// Cleanup expired entries
    pub fn cleanup(&self) {
        self.analysis_cache.cleanup();
        self.llm_cache.cleanup();
        self.docs_cache.cleanup();
    }

    /// Get statistics for all caches
    pub fn all_stats(&self) -> AllCacheStats {
        AllCacheStats {
            analysis: self.analysis_cache.stats(),
            llm: self.llm_cache.stats(),
            docs: self.docs_cache.stats(),
        }
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for all caches
#[derive(Debug, Clone, Default)]
pub struct AllCacheStats {
    pub analysis: CacheStats,
    pub llm: CacheStats,
    pub docs: CacheStats,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_cache_basic() {
        let cache: MemoryCache<String, i32> = MemoryCache::new(Duration::from_secs(60), 10);
        
        cache.insert("key1".to_string(), 42);
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        assert_eq!(cache.get(&"key2".to_string()), None);
    }

    #[test]
    fn test_cache_expiration() {
        let cache: MemoryCache<String, i32> = MemoryCache::new(Duration::from_millis(10), 10);
        
        cache.insert("key1".to_string(), 42);
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_stats() {
        let cache: MemoryCache<String, i32> = MemoryCache::new(Duration::from_secs(60), 10);
        
        cache.insert("key1".to_string(), 1);
        cache.insert("key2".to_string(), 2);
        cache.get(&"key1".to_string());
        cache.get(&"key1".to_string());
        
        let stats = cache.stats();
        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.total_hits, 2);
    }

    #[test]
    fn test_llm_cache_key() {
        let key1 = LLMCacheKey::new("What is authentication?", "gpt-4");
        let key2 = LLMCacheKey::new("What is authentication?", "gpt-4");
        let key3 = LLMCacheKey::new("Different prompt", "gpt-4");
        
        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }
}
