#![crate_name = "foo"]

// TODO::: rm it
// has foo/struct.WithSelf.html '//a/@href' '../foo/struct.WithSelf.html#method.force_failure'
// @has foo/index.html '//a/@href' '../foo/struct.Foo.html#method.new'
// @has foo/struct.Foo.html '//a/@href' '../foo/struct.Foo.html#method.new'

/// Use [`new`] to create a new instance.
///
/// [`new`]: Self::new
pub struct Foo;

impl Foo {
    pub fn new() -> Self {
        unimplemented!()
    }
}

// @has foo/index.html '//a/@href' '../foo/struct.Bar.html#method.new2'
// @has foo/struct.Bar.html '//a/@href' '../foo/struct.Bar.html#method.new2'

/// Use [`new2`] to create a new instance.
///
/// [`new2`]: Self::new2
pub struct Bar;

impl Bar {
    pub fn new2() -> Self {
        unimplemented!()
    }
}

// TODO::: make it seperatly because it inteferes with item inside impl
// has foo/index.html '//a/@href' '../foo/struct.WithSelf.html#structfield.u8_field'
// /// [`u8_field`]
// ///
// /// [`u8_field`]: Self::u8_field
pub struct WithSelf {
// @has foo/struct.WithSelf.html '//a/@href' '../foo/struct.WithSelf.html#structfield.u8_field'
    /// [`u8_field`]
    ///
    // TODO::: fix it
    // /// [`u8_field`]: Self::u8_field
    /// [`u8_field`]: WithSelf::u8_field
    pub u8_field: u8
}

impl WithSelf {
// @has foo/struct.WithSelf.html '//a/@href' '../foo/struct.WithSelf.html#method.basic'
// TODO::: rm it
// @has foo/struct.WithSelf.html '//a/@href' '../foo/struct.WithSelf.html#method.basic_par'
// @has foo/struct.WithSelf.html '//a/@href' '../foo/struct.WithSelf.html#method.basic_prefix'
    /// [`basic`]
    ///
    /// [`basic`]: Self::basic
    pub fn basic() -> Self {
        unimplemented!()
    }

    /// [`basic_par`]
    ///
    /// [`basic_par`]: Self::basic_par()
    pub fn basic_par() -> Self {
        unimplemented!()
    }

    /// [`basic_prefix`]
    ///
    /// [`basic_prefix`]: method@Self::basic_prefix
    pub fn basic_prefix() -> Self {
        unimplemented!()
    }

// @has foo/struct.WithSelf.html '//a/@href' '../foo/struct.WithSelf.html#method.name_self'
// TODO::: rm it
// @has foo/struct.WithSelf.html '//a/@href' '../foo/struct.WithSelf.html#method.name_self_par'
// @has foo/struct.WithSelf.html '//a/@href' '../foo/struct.WithSelf.html#method.name_self_prefix'

    /// [`name_self`]
    ///
    /// [`name_self`]: WithSelf::name_self
    pub fn name_self() -> Self {
        unimplemented!()
    }

    /// [`name_self_par`]
    ///
    /// [`name_self_par`]: WithSelf::name_self_par()
    pub fn name_self_par() -> Self {
        unimplemented!()
    }

    /// [`name_self_prefix`]
    ///
    /// [`name_self_prefix`]: method@WithSelf::name_self_prefix
    pub fn name_self_prefix() -> Self {
        unimplemented!()
    }
}
