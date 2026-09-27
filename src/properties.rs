use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Properties {
    pub values: HashMap<String, String>,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set<K: Into<String>, V: Into<String>>(&mut self, key: K, val: V) {
        self.values.insert(key.into(), val.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    pub fn get_int(&self, key: &str, default: i64) -> i64 {
        self.get(key).and_then(|s| s.parse().ok()).unwrap_or(default)
    }

    pub fn get_float(&self, key: &str, default: f64) -> f64 {
        self.get(key).and_then(|s| s.parse().ok()).unwrap_or(default)
    }

    /// Evaluates animated keyframe string: "0=0.0;30=1.0;60=0.5"
    pub fn get_animated_float(&self, key: &str, position: i64, default: f64) -> f64 {
        let val_str = match self.get(key) {
            Some(s) => s,
            None => return default,
        };

        if !val_str.contains(';') && !val_str.contains('=') {
            return val_str.parse().unwrap_or(default);
        }

        // Parse keyframes
        let mut keyframes: Vec<(i64, f64)> = Vec::new();
        for part in val_str.split(';') {
            let pair: Vec<&str> = part.split('=').collect();
            if pair.len() == 2 {
                if let (Ok(f), Ok(v)) = (pair[0].trim().parse::<i64>(), pair[1].trim().parse::<f64>()) {
                    keyframes.push((f, v));
                }
            }
        }

        if keyframes.is_empty() {
            return default;
        }

        keyframes.sort_by_key(|&(f, _)| f);

        // Before first keyframe
        if position <= keyframes[0].0 {
            return keyframes[0].1;
        }

        // After last keyframe
        if position >= keyframes[keyframes.len() - 1].0 {
            return keyframes[keyframes.len() - 1].1;
        }

        // Interpolate between two keyframes
        for i in 0..keyframes.len() - 1 {
            let (f0, v0) = keyframes[i];
            let (f1, v1) = keyframes[i + 1];
            if position >= f0 && position <= f1 {
                let t = (position - f0) as f64 / (f1 - f0).max(1) as f64;
                return v0 + t * (v1 - v0);
            }
        }

        default
    }
}
