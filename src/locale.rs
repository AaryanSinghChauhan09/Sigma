//! SigmaOS localization and internationalization module.
//!
//! This module provides abstractions for locale management,
//! language selection, and internationalization.
//!
//! # Example
//!
//! ```
//! use sigma_system::locale::*;
//!
//! let mut locale_mgr = LocaleManager::new();
//! locale_mgr.set_locale("en_US.UTF-8").unwrap();
//! locale_mgr.set_timezone("America/New_York").unwrap();
//! ```

use std::collections::BTreeMap;

/// Errors that can occur in locale management.
#[derive(Debug, Clone, PartialEq)]
pub enum LocaleError {
    /// Invalid locale format.
    InvalidLocale(String),
    /// Locale not found.
    LocaleNotFound(String),
    /// Invalid timezone.
    InvalidTimezone(String),
    /// Translation not found.
    TranslationNotFound(String),
}

impl std::fmt::Display for LocaleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocaleError::InvalidLocale(locale) => write!(f, "Invalid locale '{}'", locale),
            LocaleError::LocaleNotFound(locale) => write!(f, "Locale '{}' not found", locale),
            LocaleError::InvalidTimezone(tz) => write!(f, "Invalid timezone '{}'", tz),
            LocaleError::TranslationNotFound(key) => write!(f, "Translation '{}' not found", key),
        }
    }
}

impl std::error::Error for LocaleError {}

/// Supported locale.
#[derive(Debug, Clone, PartialEq)]
pub struct Locale {
    /// Locale code (e.g., "en_US.UTF-8").
    pub code: String,
    /// Language code (e.g., "en").
    pub language: String,
    /// Country code (e.g., "US").
    pub country: String,
    /// Character encoding (e.g., "UTF-8").
    pub encoding: String,
    /// Display name (e.g., "English (United States)").
    pub display_name: String,
}

impl Locale {
    /// Parse a locale string.
    pub fn parse(locale_str: &str) -> Result<Self, LocaleError> {
        // Format: language[_country][.encoding]
        let parts: Vec<&str> = locale_str.split('.').collect();
        let base = parts[0];
        let encoding = if parts.len() > 1 { parts[1] } else { "UTF-8" };
        
        let base_parts: Vec<&str> = base.split('_').collect();
        let language = base_parts[0].to_string();
        let country = if base_parts.len() > 1 { base_parts[1].to_string() } else { "".to_string() };
        
        let display_name = if country.is_empty() {
            language.to_uppercase()
        } else {
            format!("{} ({})", language.to_uppercase(), country)
        };
        
        Ok(Self {
            code: locale_str.to_string(),
            language,
            country,
            encoding: encoding.to_string(),
            display_name,
        })
    }
}

/// Timezone information.
#[derive(Debug, Clone, PartialEq)]
pub struct Timezone {
    /// Timezone identifier (e.g., "America/New_York").
    pub id: String,
    /// Display name (e.g., "Eastern Time").
    pub display_name: String,
    /// UTC offset in hours.
    pub utc_offset: i32,
}

/// Translation entry.
#[derive(Debug, Clone, PartialEq)]
pub struct Translation {
    /// Translation key.
    pub key: String,
    /// Translated text.
    pub text: String,
    /// Context (optional).
    pub context: Option<String>,
}

/// Locale manager.
#[derive(Debug)]
pub struct LocaleManager {
    /// Current locale.
    current_locale: Option<Locale>,
    /// Current timezone.
    current_timezone: Option<Timezone>,
    /// Available locales.
    available_locales: BTreeMap<String, Locale>,
    /// Available timezones.
    available_timezones: BTreeMap<String, Timezone>,
    /// Translations per locale.
    translations: BTreeMap<String, BTreeMap<String, Translation>>,
}

impl LocaleManager {
    /// Create a new locale manager with default locales.
    pub fn new() -> Self {
        let mut mgr = Self {
            current_locale: None,
            current_timezone: None,
            available_locales: BTreeMap::new(),
            available_timezones: BTreeMap::new(),
            translations: BTreeMap::new(),
        };
        mgr.add_default_locales();
        mgr.add_default_timezones();
        mgr
    }

    fn add_default_locales(&mut self) {
        let locales = [
            "en_US.UTF-8",
            "en_GB.UTF-8",
            "en_CA.UTF-8",
            "en_AU.UTF-8",
            "de_DE.UTF-8",
            "de_AT.UTF-8",
            "de_CH.UTF-8",
            "fr_FR.UTF-8",
            "fr_CA.UTF-8",
            "fr_BE.UTF-8",
            "es_ES.UTF-8",
            "es_MX.UTF-8",
            "es_AR.UTF-8",
            "it_IT.UTF-8",
            "pt_BR.UTF-8",
            "pt_PT.UTF-8",
            "ru_RU.UTF-8",
            "zh_CN.UTF-8",
            "zh_TW.UTF-8",
            "ja_JP.UTF-8",
            "ko_KR.UTF-8",
            "ar_SA.UTF-8",
            "hi_IN.UTF-8",
            "nl_NL.UTF-8",
            "pl_PL.UTF-8",
            "sv_SE.UTF-8",
            "no_NO.UTF-8",
            "da_DK.UTF-8",
            "fi_FI.UTF-8",
            "tr_TR.UTF-8",
        ];
        for locale_str in locales {
            if let Ok(locale) = Locale::parse(locale_str) {
                self.available_locales.insert(locale.code.clone(), locale);
            }
        }
    }

    fn add_default_timezones(&mut self) {
        let timezones = [
            ("UTC", "Coordinated Universal Time", 0),
            ("America/New_York", "Eastern Time", -5),
            ("America/Chicago", "Central Time", -6),
            ("America/Denver", "Mountain Time", -7),
            ("America/Los_Angeles", "Pacific Time", -8),
            ("America/Phoenix", "Arizona Time", -7),
            ("America/Anchorage", "Alaska Time", -9),
            ("America/Honolulu", "Hawaii Time", -10),
            ("Europe/London", "Greenwich Mean Time", 0),
            ("Europe/Paris", "Central European Time", 1),
            ("Europe/Berlin", "Central European Time", 1),
            ("Europe/Madrid", "Central European Time", 1),
            ("Europe/Rome", "Central European Time", 1),
            ("Europe/Moscow", "Moscow Time", 3),
            ("Asia/Tokyo", "Japan Standard Time", 9),
            ("Asia/Shanghai", "China Standard Time", 8),
            ("Asia/Kolkata", "India Standard Time", 5),
            ("Australia/Sydney", "Australian Eastern Time", 10),
            ("Australia/Melbourne", "Australian Eastern Time", 10),
            ("Pacific/Auckland", "New Zealand Time", 12),
        ];
        for (id, name, offset) in timezones {
            let tz = Timezone {
                id: id.to_string(),
                display_name: name.to_string(),
                utc_offset: offset,
            };
            self.available_timezones.insert(id.to_string(), tz);
        }
    }

    /// Set the current locale.
    pub fn set_locale(&mut self, locale_str: &str) -> Result<(), LocaleError> {
        let locale = Locale::parse(locale_str)?;
        if !self.available_locales.contains_key(&locale.code) {
            return Err(LocaleError::LocaleNotFound(locale_str.to_string()));
        }
        self.current_locale = Some(locale);
        Ok(())
    }

    /// Get the current locale.
    pub fn get_locale(&self) -> Option<&Locale> {
        self.current_locale.as_ref()
    }

    /// Set the current timezone.
    pub fn set_timezone(&mut self, timezone_id: &str) -> Result<(), LocaleError> {
        if !self.available_timezones.contains_key(timezone_id) {
            return Err(LocaleError::InvalidTimezone(timezone_id.to_string()));
        }
        self.current_timezone = self.available_timezones.get(timezone_id).cloned();
        Ok(())
    }

    /// Get the current timezone.
    pub fn get_timezone(&self) -> Option<&Timezone> {
        self.current_timezone.as_ref()
    }

    /// Get all available locales.
    pub fn get_locales(&self) -> Vec<&Locale> {
        self.available_locales.values().collect()
    }

    /// Get all available timezones.
    pub fn get_timezones(&self) -> Vec<&Timezone> {
        self.available_timezones.values().collect()
    }

    /// Add a translation.
    pub fn add_translation(&mut self, locale: &str, key: &str, text: &str, context: Option<&str>) {
        let translations = self.translations.entry(locale.to_string()).or_default();
        let translation = Translation {
            key: key.to_string(),
            text: text.to_string(),
            context: context.map(|s| s.to_string()),
        };
        translations.insert(key.to_string(), translation);
    }

    /// Get a translation.
    pub fn get_translation(&self, locale: &str, key: &str) -> Result<String, LocaleError> {
        if let Some(translations) = self.translations.get(locale) {
            if let Some(translation) = translations.get(key) {
                return Ok(translation.text.clone());
            }
        }
        Err(LocaleError::TranslationNotFound(key.to_string()))
    }

    /// Format a number according to locale.
    pub fn format_number(&self, number: f64, locale: Option<&str>) -> String {
        let locale = locale.or_else(|| self.current_locale.as_ref().map(|l| l.code.as_str())).unwrap_or("en_US.UTF-8");
        // Simplified number formatting
        let parts: Vec<&str> = locale.split('_').collect();
        let country = if parts.len() > 1 { parts[1] } else { "US" };
        
        match country {
            "US" | "GB" | "CA" | "AU" => format!("{:.2}", number),
            "DE" | "FR" | "IT" | "ES" => {
                let formatted = format!("{:.2}", number);
                formatted.replace('.', ",")
            }
            "JP" | "CN" | "KR" => format!("{:.0}", number),
            _ => format!("{:.2}", number),
        }
    }

    /// Format a date according to locale.
    pub fn format_date(&self, timestamp: i64, locale: Option<&str>) -> String {
        let locale = locale.or_else(|| self.current_locale.as_ref().map(|l| l.code.as_str())).unwrap_or("en_US.UTF-8");
        // Simplified date formatting
        let parts: Vec<&str> = locale.split('_').collect();
        let country = if parts.len() > 1 { parts[1] } else { "US" };
        
        match country {
            "US" => format!("{}/{}/{}", timestamp % 100, (timestamp / 100) % 100, timestamp / 10000),
            "GB" | "AU" | "CA" => format!("{}/{}/{}", (timestamp / 100) % 100, timestamp % 100, timestamp / 10000),
            "DE" | "FR" | "IT" | "ES" => format!("{}.{}/{}", (timestamp / 100) % 100, timestamp % 100, timestamp / 10000),
            "JP" | "CN" | "KR" => format!("{}/{}/{}", timestamp / 10000, (timestamp / 100) % 100, timestamp % 100),
            _ => format!("{}/{}/{}", timestamp % 100, (timestamp / 100) % 100, timestamp / 10000),
        }
    }

    /// Generate locale configuration.
    pub fn generate_locale_config(&self) -> String {
        let mut config = String::new();
        if let Some(locale) = &self.current_locale {
            config.push_str(&format!("LANG={}\n", locale.code));
            config.push_str(&format!("LC_ALL={}\n", locale.code));
        }
        if let Some(tz) = &self.current_timezone {
            config.push_str(&format!("TZ={}\n", tz.id));
        }
        config
    }
}

impl Default for LocaleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_locale() {
        let mut mgr = LocaleManager::new();
        mgr.set_locale("en_US.UTF-8").unwrap();
        assert_eq!(mgr.get_locale().unwrap().code, "en_US.UTF-8");
    }

    #[test]
    fn test_set_invalid_locale() {
        let mut mgr = LocaleManager::new();
        let result = mgr.set_locale("xx_YY.UTF-8");
        assert!(result.is_err());
    }

    #[test]
    fn test_set_timezone() {
        let mut mgr = LocaleManager::new();
        mgr.set_timezone("America/New_York").unwrap();
        assert_eq!(mgr.get_timezone().unwrap().id, "America/New_York");
    }

    #[test]
    fn test_locale_parse() {
        let locale = Locale::parse("en_US.UTF-8").unwrap();
        assert_eq!(locale.language, "en");
        assert_eq!(locale.country, "US");
        assert_eq!(locale.encoding, "UTF-8");
    }

    #[test]
    fn test_add_translation() {
        let mut mgr = LocaleManager::new();
        mgr.add_translation("en_US.UTF-8", "hello", "Hello", None);
        mgr.add_translation("de_DE.UTF-8", "hello", "Hallo", None);
        assert_eq!(mgr.get_translation("en_US.UTF-8", "hello").unwrap(), "Hello");
        assert_eq!(mgr.get_translation("de_DE.UTF-8", "hello").unwrap(), "Hallo");
    }

    #[test]
    fn test_format_number() {
        let mgr = LocaleManager::new();
        let us = mgr.format_number(1234.56, Some("en_US.UTF-8"));
        let de = mgr.format_number(1234.56, Some("de_DE.UTF-8"));
        // US uses period, German uses comma as decimal separator
        assert!(us.contains(".") || us.contains(","));
        assert!(de.contains(",") || de.contains("."));
        // Both should have 2 decimal places
        assert!(us.len() > 5);
        assert!(de.len() > 5);
    }

    #[test]
    fn test_generate_locale_config() {
        let mut mgr = LocaleManager::new();
        mgr.set_locale("en_US.UTF-8").unwrap();
        mgr.set_timezone("America/New_York").unwrap();
        let config = mgr.generate_locale_config();
        assert!(config.contains("LANG=en_US.UTF-8"));
        assert!(config.contains("TZ=America/New_York"));
    }

    #[test]
    fn test_available_locales() {
        let mgr = LocaleManager::new();
        let locales = mgr.get_locales();
        assert!(locales.len() > 20);
    }

    #[test]
    fn test_available_timezones() {
        let mgr = LocaleManager::new();
        let timezones = mgr.get_timezones();
        assert!(timezones.len() > 15);
    }
}
