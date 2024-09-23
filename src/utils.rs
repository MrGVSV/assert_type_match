use syn::{Attribute, Field, Member};

/// Extracts all `cfg` attributes from the given list of attributes.
pub(crate) fn extract_cfg_attrs(attrs: &[Attribute]) -> impl Iterator<Item = &Attribute> {
    attrs.iter().filter(|attr| attr.path().is_ident("cfg"))
}

/// Creates a [`Member`] from the given field and index.
pub(crate) fn create_member(field: &Field, index: usize) -> Member {
    field
        .ident
        .clone()
        .map(Member::from)
        .unwrap_or_else(|| Member::from(index))
}
