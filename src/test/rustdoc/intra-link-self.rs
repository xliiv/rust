#![crate_name = "foo"]

// TODO
// https://doc.rust-lang.org/reference/items/structs.html
// https://doc.rust-lang.org/reference/items/enumerations.html
// https://doc.rust-lang.org/reference/items/unions.html
// https://doc.rust-lang.org/reference/items/traits.html
// https://doc.rust-lang.org/reference/items/implementations.html
// https://doc.rust-lang.org/reference/items/associated-items.html

// TODO::: rm it
// has foo/struct.WithSelf.html '//a/@href' '../foo/struct.WithSelf.html#method.force_failure'
// has foo/index.html '//a/@href' '../foo/struct.Foo.html#method.new'
// has foo/struct.Foo.html '//a/@href' '../foo/struct.Foo.html#method.new'

//TODO: better name for bad, Bad, etc.
//TODO: do we need ok, Ok, OK variants?
//TODO: verfify that all <at>has works correctly
//TODO: use unique names instead of bad for field, method, etc.

pub struct MyStruct {
    //TODO fix it
    /// [`struct_field_ok`]
    ///
    /// [`struct_field_ok`]: MyStruct::struct_field_ok
    pub struct_field_ok: u8,

    // @has foo/struct.MyStruct.html '//a/@href' '../foo/struct.MyStruct.html#structfield.struct_field_bad'

    /// [`struct_field_bad`]
    ///
    /// [`struct_field_bad`]: Self::struct_field_bad
    pub struct_field_bad: u8,
}

pub enum MyEnum {
    /// [`EnumVariantOk`]
    ///
    /// [`EnumVariantOk`]: MyEnum::EnumVariantOk
    EnumVariantOk,

    // @has foo/enum.MyEnum.html '//a/@href' '../foo/enum.MyEnum.html#EnumVariantBad.v'

    /// [`EnumVariantBad`]
    ///
    /// [`EnumVariantBad`]: Self::EnumVariantBad
    EnumVariantBad,
}

pub union MyUnion {
    /// [`union_field_ok`]
    ///
    /// [`union_field_ok`]: MyUnion::union_field_ok
    pub union_field_ok: f32,

    // @has foo/union.MyUnion.html '//a/@href' '../foo/union.MyUnion.html#structfield.union_field_bad'

    /// [`union_field_bad`]
    ///
    /// [`union_field_bad`]: Self::union_field_bad
    pub union_field_bad: f32,
}

pub trait MyTrait {
    //TODO:
    /// [`TRAIT_CONST_OK`]
    ///
    /// [`TRAIT_CONST_OK`]: MyTrait::TRAIT_CONST_OK
    const TRAIT_CONST_OK: i32 = 1;

    // @has foo/trait.MyTrait.html '//a/@href' '../foo/trait.MyTrait.html#associatedconstant.TRAIT_CONST_BAD'

    /// [`TRAIT_CONST_BAD`]
    ///
    /// [`TRAIT_CONST_BAD`]: Self::TRAIT_CONST_BAD
    const TRAIT_CONST_BAD: i32 = 1;

    /// [`trait_method_ok`]
    ///
    /// [`trait_method_ok`]: MyTrait::trait_method_ok
    fn trait_method_ok() {}

    // @has foo/trait.MyTrait.html '//a/@href' '../foo/trait.MyTrait.html#method.trait_method_bad'

    /// [`trait_method_bad`]
    ///
    /// [`trait_method_bad`]: Self::trait_method_bad
    fn trait_method_bad() {}
}

impl MyStruct {
    /// [`impl_ok`]
    ///
    /// [`impl_ok`]: MyStruct::impl_ok
    pub fn impl_ok() -> Self {
        unimplemented!()
    }

    // @has foo/struct.MyStruct.html '//a/@href' '../foo/struct.MyStruct.html#method.impl_bad'

    /// [`impl_bad`]
    ///
    /// [`impl_bad`]: Self::impl_bad
    pub fn impl_bad() -> Self {
        unimplemented!()
    }
}

impl MyTrait for MyStruct {
    // @has foo/struct.MyStruct.html '//a/@href' '../foo/trait.MyTrait.html#method.trait_method_ok'

    /// [`trait_method_ok`]
    ///
    //TODO: link is broken here
    /// [`trait_method_ok`]: MyStruct::trait_method_ok
    //TODO: this points to trait, but should link to itself like
    // trait_method_ok.v
    // instead of current
    // ../foo/trait.MyTrait.html#method.trait_method_ok
    fn trait_method_ok() {
        unimplemented!()
    }

    // @has foo/struct.MyStruct.html '//a/@href' '../foo/trait.MyTrait.html#method.trait_method_bad'

    /// [`trait_method_bad`]
    ///
    //TODO: link is broken here
    /// [`trait_method_bad`]: Self::trait_method_bad
    //TODO: this points to trait, but should link to itself like
    // trait_method_bad.v
    // instead of current
    // ../foo/trait.MyTrait.html#method.trait_method_bad
    fn trait_method_bad() {
        unimplemented!()
    }
}

//trait AssociatedType {
//    ///
//    type Assoc;
//    const ID: i32;
//
//    pub fn ok() -> Self;
//    pub fn bad() -> Self;
//
//}
////TODO:
//impl AssociatedType for MyStruct {
//    /// [`type`]
//    ///
//    // TODO:
//    /// [`type`]: MyStruct::Assoc
//    type Assoc = u32;
//
//    /// [`const`]
//    ///
//    // TODO:
//    /// [`const`]: MyStruct::Assoc
//    const ID: i32 = 5;
//
//    /// [`ok`]
//    ///
//    /// [`ok`]: MyStruct::ok
//    pub fn ok() -> Self {
//        unimplemented!()
//    }
//
//    /// [`bad`]
//    ///
//    /// [`bad`]: Self::bad
//    pub fn bad() -> Self {
//        unimplemented!()
//    }
//}












// OLD STUFF

///// Use [`new`] to create a new instance.
/////
///// [`new`]: Self::new
//pub struct Foo;
//
//impl Foo {
//    pub fn new() -> Self {
//        unimplemented!()
//    }
//}
//
//// has foo/index.html '//a/@href' '../foo/struct.Bar.html#method.new2'
//// has foo/struct.Bar.html '//a/@href' '../foo/struct.Bar.html#method.new2'
//
///// Use [`new2`] to create a new instance.
/////
///// [`new2`]: Self::new2
//pub struct Bar;
//
//impl Bar {
//    pub fn new2() -> Self {
//        unimplemented!()
//    }
//}
//
//// TODO::: make it seperatly because it inteferes with item inside impl
//// has foo/index.html '//a/@href' '../foo/struct.WithSelf.html#structfield.u8_field'
//// /// [`u8_field`]
//// ///
//// /// [`u8_field`]: Self::u8_field
//pub struct WithSelf {
//
//// has foo/struct.WithSelf.html '//a/@href' '../foo/struct.WithSelf.html#structfield.u8_field'
//
//    /// [`u8_field`]
//    ///
//    // TODO::: make it work
//    /// [`u8_field`]: Self::u8_field
//    // /// [`u8_field`]: WithSelf::u8_field
//    pub u8_field: u8
//}
//
//impl WithSelf {
//
//// has foo/struct.WithSelf.html '//a/@href' '../foo/struct.WithSelf.html#method.basic'
//
//    /// [`basic`]
//    ///
//    /// [`basic`]: Self::basic
//    pub fn basic() {
//        unimplemented!()
//    }
//
//// has foo/struct.WithSelf.html '//a/@href' '../foo/struct.WithSelf.html#method.name_self'
//
//    /// [`name_self`]
//    ///
//    /// [`name_self`]: WithSelf::name_self
//    pub fn name_self() {
//        unimplemented!()
//    }
//}

