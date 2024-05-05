use std::ops::{BitAnd, BitOr, BitXor};

use crate::style::Attribute;

/// a bitset for all possible attributes
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Attributes(u32);

impl From<Attribute> for Attributes {
    fn from(attribute: Attribute) -> Self {
        Self(attribute.bytes())
    }
}

impl From<&[Attribute]> for Attributes {
    fn from(arr: &[Attribute]) -> Self {
        let mut attributes = Attributes::default();
        for &attr in arr {
            attributes.set(attr);
        }
        attributes
    }
}

impl BitAnd<Attribute> for Attributes {
    type Output = Self;
    fn bitand(self, rhs: Attribute) -> Self {
        Self(self.0 & rhs.bytes())
    }
}
impl BitAnd for Attributes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitOr<Attribute> for Attributes {
    type Output = Self;
    fn bitor(self, rhs: Attribute) -> Self {
        Self(self.0 | rhs.bytes())
    }
}
impl BitOr for Attributes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitXor<Attribute> for Attributes {
    type Output = Self;
    fn bitxor(self, rhs: Attribute) -> Self {
        Self(self.0 ^ rhs.bytes())
    }
}
impl BitXor for Attributes {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl Attributes {
    /// Returns the empty bitset.
    #[inline(always)]
    pub const fn none() -> Self {
        Self(0)
    }

    /// Returns a copy of the bitset with the given attribute set.
    /// If it's already set, this returns the bitset unmodified.
    #[inline(always)]
    pub const fn with(self, attribute: Attribute) -> Self {
        Self(self.0 | attribute.bytes())
    }

    /// Returns a copy of the bitset with the given attribute unset.
    /// If it's not set, this returns the bitset unmodified.
    #[inline(always)]
    pub const fn without(self, attribute: Attribute) -> Self {
        Self(self.0 & !attribute.bytes())
    }

    /// Sets the attribute.
    /// If it's already set, this does nothing.
    #[inline(always)]
    pub fn set(&mut self, attribute: Attribute) {
        self.0 |= attribute.bytes();
    }

    /// Unsets the attribute.
    /// If it's not set, this changes nothing.
    #[inline(always)]
    pub fn unset(&mut self, attribute: Attribute) {
        self.0 &= !attribute.bytes();
    }

    /// Sets the attribute if it's unset, unset it
    /// if it is set.
    #[inline(always)]
    pub fn toggle(&mut self, attribute: Attribute) {
        self.0 ^= attribute.bytes();
    }

    /// Returns whether the attribute is set.
    #[inline(always)]
    pub const fn has(self, attribute: Attribute) -> bool {
        self.0 & attribute.bytes() != 0
    }

    /// Sets all the passed attributes. Removes none.
    #[inline(always)]
    pub fn extend(&mut self, attributes: Attributes) {
        self.0 |= attributes.0;
    }

    /// Returns whether there is no attribute set.
    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if there are attributes common to both sets.
    #[inline(always)]
    pub const fn intersects(&self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    /// Returns `true` if all attributes in `other` are contained within `self`.
    #[inline(always)]
    pub const fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Returns the attributes contained in *both* `self` and `other`.
    ///
    /// This is equivalent to using the `&` operator.
    #[inline(always)]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Returns the combined attributes of `self` and `other`.
    ///
    /// This is equivalent to using the `|` operator.
    #[inline(always)]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns the attributes present in `self` that are not present in `other`.
    #[inline(always)]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// Returns the attributes present in `self` or `other`, but not present in
    /// both.
    ///
    /// This is equivalent to using the `^` operator.
    #[inline(always)]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Returns an iterator over all the attributes in `self`.
    pub fn iter(&self) -> impl Iterator<Item = Attribute> + '_ {
        Attribute::iterator().filter(|a| self.has(*a))
    }
}

#[cfg(test)]
mod tests {
    use super::{Attribute, Attributes};

    #[test]
    fn test_attributes() {
        let mut attributes: Attributes = Attribute::Bold.into();
        assert!(attributes.has(Attribute::Bold));
        attributes.set(Attribute::Italic);
        assert!(attributes.has(Attribute::Italic));
        attributes.unset(Attribute::Italic);
        assert!(!attributes.has(Attribute::Italic));
        attributes.toggle(Attribute::Bold);
        assert!(attributes.is_empty());
    }

    #[test]
    fn test_attributes_const() {
        const ATTRIBUTES: Attributes = Attributes::none()
            .with(Attribute::Bold)
            .with(Attribute::Italic)
            .without(Attribute::Bold);
        assert!(!ATTRIBUTES.has(Attribute::Bold));
        assert!(ATTRIBUTES.has(Attribute::Italic));
    }

    #[test]
    fn test_set_operations() {
        use Attribute::*;
        let a = Attributes::none()
            .with(Bold)
            .with(Italic)
            .with(Dim)
            .with(Undercurled);

        let a_subset = Attributes::none().with(Bold).with(Dim);
        let b = Attributes::none()
            .with(Bold)
            .with(Reverse)
            .with(Dim)
            .with(Underdashed);

        assert!(a.contains(a));
        assert!(b.contains(b));

        assert!(a.contains(a_subset));
        assert!(!a_subset.contains(a));
        assert!(!a.contains(b));
        assert!(!b.contains(a));

        assert!(a.intersects(b));
        assert!(b.intersects(a));

        let a_b_common = Attributes::from([Bold, Dim].as_slice());
        assert_eq!(a.intersection(b), a_b_common);
        assert_eq!(b.intersection(a), a_b_common);

        let a_b_union = Attributes::none()
            .with(Bold)
            .with(Italic)
            .with(Dim)
            .with(Undercurled)
            .with(Reverse)
            .with(Underdashed);

        assert_eq!(a.union(b), a_b_union);

        let a_b_diff = Attributes::none().with(Italic).with(Undercurled);
        let b_a_diff = Attributes::none().with(Reverse).with(Underdashed);
        assert_eq!(a.difference(b), a_b_diff);
        assert_eq!(b.difference(a), b_a_diff);

        let a_b_symdiff = Attributes::none()
            // in a
            .with(Italic)
            .with(Undercurled)
            // in b
            .with(Reverse)
            .with(Underdashed);
        assert_eq!(a.symmetric_difference(b), a_b_symdiff);
        assert_eq!(b.symmetric_difference(a), a_b_symdiff);
        assert_eq!(a.symmetric_difference(b), a_b_diff.union(b_a_diff));

        // empty sets
        let empty_a = Attributes::none();
        let empty_b = Attributes::none();
        assert!(!empty_a.intersects(empty_b))
    }
}
