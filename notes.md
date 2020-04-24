# Struct

[src/librustdoc/fold.rs:28] &i = Struct {
    struct_type: Plain,
    generics: Generics {
        params: [],
        where_predicates: [],
    },
    fields: [
        Item {
            source: Span {
                filename: Real(
                    "/home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs",
                ),
                cnum: crate0,
                loline: 13,
                locol: 4,
                hiline: 13,
                hicol: 23,
                original: /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:13:5: 13:24,
            },
            name: Some(
                "field_works",
            ),
            attrs: Attributes {
                doc_strings: [
                    SugaredDoc(
                        0,
                        /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:10:5: 10:24,
                        " [`field_works`]",
                    ),
                    SugaredDoc(
                        1,
                        /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:11:5: 11:8,
                        "",
                    ),
                    SugaredDoc(
                        1,
                        /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:12:5: 12:47,
                        " [`field_works`]: MyStruct::field_works",
                    ),
                ],
                other_attrs: [],
                cfg: None,
                span: Some(
                    /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:10:5: 10:24,
                ),
                links: [],
                inner_docs: false,
                impl_name: None,
            },
            inner: StructFieldItem(
                Primitive(
                    U8,
                ),
            ),
            visibility: Public,
            def_id: DefId(0:4 ~ foo[8787]::MyStruct[0]::field_works[0]),
            stability: None,
            deprecation: None,
        },
        Item {
            source: Span {
                filename: Real(
                    "/home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs",
                ),
                cnum: crate0,
                loline: 18,
                locol: 4,
                hiline: 18,
                hicol: 24,
                original: /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:18:5: 18:25,
            },
            name: Some(
                "field_broken",
            ),
            attrs: Attributes {
                doc_strings: [
                    SugaredDoc(
                        0,
                        /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:15:5: 15:25,
                        " [`field_broken`]",
                    ),
                    SugaredDoc(
                        1,
                        /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:16:5: 16:8,
                        "",
                    ),
                    SugaredDoc(
                        1,
                        /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:17:5: 17:45,
                        " [`field_broken`]: Self::field_broken",
                    ),
                ],
                other_attrs: [],
                cfg: None,
                span: Some(
                    /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:15:5: 15:25,
                ),
                links: [],
                inner_docs: false,
                impl_name: None,
            },
            inner: StructFieldItem(
                Primitive(
                    U8,
                ),
            ),
            visibility: Public,
            def_id: DefId(0:5 ~ foo[8787]::MyStruct[0]::field_broken[0]),
            stability: None,
            deprecation: None,
        },
    ],
    fields_stripped: false,
}
