#![crate_name = "foo"]

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

// TODO
// https://doc.rust-lang.org/reference/items/structs.html
// https://doc.rust-lang.org/reference/items/enumerations.html
// https://doc.rust-lang.org/reference/items/unions.html
// https://doc.rust-lang.org/reference/items/traits.html
// https://doc.rust-lang.org/reference/items/implementations.html
// https://doc.rust-lang.org/reference/items/associated-items.html

// TODO::: rm it
// has foo/struct.WithSelf.html '//a/@href' '../foo/struct.WithSelf.html#method.force_failure'
//TODO: better name for bad, Bad, etc.

pub struct MyStruct {
    // @has foo/struct.MyStruct.html '//a/@href' '../foo/struct.MyStruct.html#structfield.struct_field_bad'

    /// [`struct_field_bad`]
    ///
    /// [`struct_field_bad`]: Self::struct_field_bad
    pub struct_field_bad: u8,
}

pub enum MyEnum {
    // @has foo/enum.MyEnum.html '//a/@href' '../foo/enum.MyEnum.html#EnumVariantBad.v'

    /// [`EnumVariantBad`]
    ///
    /// [`EnumVariantBad`]: Self::EnumVariantBad
    EnumVariantBad,
}

pub union MyUnion {
    // @has foo/union.MyUnion.html '//a/@href' '../foo/union.MyUnion.html#structfield.union_field_bad'

    /// [`union_field_bad`]
    ///
    /// [`union_field_bad`]: Self::union_field_bad
    pub union_field_bad: f32,
}

pub trait MyTrait {
    // @has foo/trait.MyTrait.html '//a/@href' '../foo/trait.MyTrait.html#associatedtype.AssoTypeBad'

    /// [`AssoTypeBad`]
    ///
    /// [`AssoTypeBad`]: Self::AssoTypeBad
    type AssoTypeBad;

    // @has foo/trait.MyTrait.html '//a/@href' '../foo/trait.MyTrait.html#associatedconstant.ASSO_CONST_BAD'

    /// [`ASSO_CONST_BAD`]
    ///
    /// [`ASSO_CONST_BAD`]: Self::ASSO_CONST_BAD
    const ASSO_CONST_BAD: i32 = 1;

    // @has foo/trait.MyTrait.html '//a/@href' '../foo/trait.MyTrait.html#method.asso_fn_bad'

    /// [`asso_fn_bad`]
    ///
    /// [`asso_fn_bad`]: Self::asso_fn_bad
    fn asso_fn_bad() {}
}

impl MyStruct {
    // @has foo/struct.MyStruct.html '//a/@href' '../foo/struct.MyStruct.html#method.impl_bad'

    /// [`impl_bad`]
    ///
    /// [`impl_bad`]: Self::impl_bad
    pub fn impl_bad() -> Self {
        unimplemented!()
    }
}

//TODO: fix it
impl MyTrait for MyStruct {
    // @has foo/struct.MyStruct.html '//a/@href' '../foo/struct.MyStruct.html#associatedtype.AssoTypeOk'

    /// [`AssoTypeOk`]
    ///
    /// [`AssoTypeOk`]: MyTrait::AssoTypeOk
    type AssoTypeOk = u32;

    // @has foo/struct.MyStruct.html '//a/@href' '../foo/struct.MyStruct.html#associatedtype.AssoTypeBad'

    /// [`AssoTypeBad`]
    ///
    /// [`AssoTypeBad`]: Self::AssoTypeBad
    type AssoTypeBad = u32;

    // @has foo/struct.MyStruct.html '//a/@href' '../foo/struct.MyStruct.html#ASSO_CONST_OK.v'

    /// [`ASSO_CONST_OK`]
    ///
    /// [`ASSO_CONST_OK`]: MyTrait::ASSO_CONST_OK
    const ASSO_CONST_OK: i32 = 10;

    // @has foo/struct.MyStruct.html '//a/@href' '../foo/struct.MyStruct.html#ASSO_CONST_BAD.v'

    /// [`ASSO_CONST_BAD`]
    ///
    /// [`ASSO_CONST_BAD`]: Self::ASSO_CONST_BAD
    const ASSO_CONST_BAD: i32 = 10;

    // @has foo/struct.MyStruct.html '//a/@href' '../foo/struct.MyStruct.html#method.asso_fn_ok'

    /// [`asso_fn_ok`]
    ///
    //TODO: link is broken here
    /// [`asso_fn_ok`]: MyStruct::asso_fn_ok
    //TODO: this points to trait, but should link to implementator like
    // asso_fn_ok.v
    // instead of current
    // ../foo/trait.MyTrait.html#method.asso_fn_ok
    fn asso_fn_ok() {
        unimplemented!()
    }

    // @has foo/struct.MyStruct.html '//a/@href' '../foo/struct.MyStruct.html#method.asso_fn_bad'

    /// [`asso_fn_bad`]
    ///
    //TODO: link is broken here
    /// [`asso_fn_bad`]: Self::asso_fn_bad
    //TODO: this points to trait, but should link to implementator like
    // asso_fn_bad.v
    // instead of current
    // ../foo/trait.MyTrait.html#method.asso_fn_bad
    fn asso_fn_bad() {
        unimplemented!()
    }
}
