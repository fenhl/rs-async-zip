// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::entry::{ZipEntry, ZipEntryBuilder};
use crate::spec::attribute::AttributeCompatibility;

/// A trait that extends [`ZipEntry`]'s functionality.
pub trait ZipEntryExt: Sized {
    /// Returns the entry's integer-based UNIX permissions.
    ///
    /// # Note
    /// This will return None if the attribute host compatibility is not listed as Unix.
    fn unix_permissions(&self) -> Option<u16>;
}

impl ZipEntryExt for ZipEntry {
    fn unix_permissions(&self) -> Option<u16> {
        if !matches!(self.attribute_compatibility, AttributeCompatibility::Unix) {
            return None;
        }

        Some(((self.external_file_attribute) >> 16) as u16)
    }
}

/// A trait that extends [`ZipEntryBuilder`]'s functionality.
pub trait ZipEntryBuilderExt {
    /// Sets the entry's Unix permissions mode.
    ///
    /// # Note
    /// This will force the entry's attribute host compatibility to Unix as well as override the previous upper
    /// sixteen bits of the entry's external file attribute (which includes any previous permissions mode).
    fn unix_permissions(self, mode: u16) -> Self;
}

impl ZipEntryBuilderExt for ZipEntryBuilder {
    fn unix_permissions(mut self, mode: u16) -> Self {
        self.0.attribute_compatibility = AttributeCompatibility::Unix;
        self.0.external_file_attribute = (self.0.external_file_attribute & 0xFFFF) | (mode as u32) << 16;
        self
    }
}
