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

pub struct MyStruct {
    /// [`ok`]
    ///
    /// [`ok`]: MyStruct::ok
    pub ok: u8,

    // @has foo/struct.MyStruct.html '//a/@href' '../foo/struct.MyStruct.html#structfield.bad'

    /// [`bad`]
    ///
    /// [`bad`]: Self::bad
    pub bad: u8,
}

pub enum MyEnum {
    /// [`Ok`]
    ///
    /// [`Ok`]: MyEnum::Ok
    Ok,

    // @has foo/enum.MyEnum.html '//a/@href' '../foo/enum.MyEnum.html#Bad.v'

    /// [`Bad`]
    ///
    /// [`Bad`]: Self::Bad
    Bad,
}

pub union MyUnion {
    /// [`ok`]
    ///
    /// [`ok`]: MyUnion::ok
    pub ok: f32,

    // @has foo/union.MyUnion.html '//a/@href' '../foo/union.MyUnion.html#structfield.bad'

    /// [`bad`]
    ///
    /// [`bad`]: Self::bad
    pub bad: f32,
}


pub trait MyTrait {
    /// [`OK`]
    ///
    /// [`OK`]: MyTrait::OK
    const OK: i32 = 1;

    // @has foo/trait.MyTrait.html '//a/@href' '../foo/trait.MyTrait.html#associatedconstant.BAD'

    /// [`BAD`]
    ///
    /// [`BAD`]: Self::BAD
    const BAD: i32 = 1;

    /// [`ok`]
    ///
    /// [`ok`]: MyTrait::ok
    fn ok() {}

    // @has foo/trait.MyTrait.html '//a/@href' '../foo/trait.MyTrait.html#method.bad'

    /// [`bad`]
    ///
    /// [`bad`]: Self::bad
    fn bad() {}
}


impl MyStruct {
    /// [`ok`]
    ///
    /// [`ok`]: MyStruct::ok
    pub fn ok() -> Self {
        unimplemented!()
    }

    // @has foo/struct.MyStruct.html '//a/@href' '../foo/struct.MyStruct.html#method.bad'

    /// [`bad`]
    ///
    /// [`bad`]: Self::bad
    pub fn bad() -> Self {
        unimplemented!()
    }
}

//impl MyTrait for MyStruct {
//    /// [`ok`]
//    ///
//    /// [`ok`]: MyStruct::ok
//    pub fn ok() -> Self {
//        unimplemented!()
//    }
//    /// [`bad`]
//    ///
//    /// [`bad`]: Self::bad
//    pub fn bad() -> Self {
//        unimplemented!()
//    }
//}
//
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

