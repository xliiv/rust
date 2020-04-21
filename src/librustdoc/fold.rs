use crate::clean::*;
use regex::Regex;

pub struct StripItem(pub Item);

lazy_static! {
    static ref SELF_MATCHER: Regex = Regex::new(r"([^A-Za-z_])Self::([a-zA-Z_])").unwrap();
}

impl StripItem {
    pub fn strip(self) -> Option<Item> {
        match self.0 {
            Item { inner: StrippedItem(..), .. } => Some(self.0),
            mut i => {
                i.inner = StrippedItem(box i.inner);
                Some(i)
            }
        }
    }
}

pub trait DocFolder: Sized {
    fn fold_item(&mut self, item: Item) -> Option<Item> {
        self.fold_item_recur(item)
    }

    /// don't override!
    fn fold_inner_recur(&mut self, inner: ItemEnum) -> ItemEnum {
        match inner {
            StrippedItem(..) => unreachable!(),
            ModuleItem(i) => ModuleItem(self.fold_mod(i)),
            StructItem(mut i) => {
                let num_fields = i.fields.len();
                i.fields = i.fields.into_iter().filter_map(|x| self.fold_item(x)).collect();
                i.fields_stripped |=
                    num_fields != i.fields.len() || i.fields.iter().any(|f| f.is_stripped());
                StructItem(i)
            }
            UnionItem(mut i) => {
                let num_fields = i.fields.len();
                i.fields = i.fields.into_iter().filter_map(|x| self.fold_item(x)).collect();
                i.fields_stripped |=
                    num_fields != i.fields.len() || i.fields.iter().any(|f| f.is_stripped());
                UnionItem(i)
            }
            EnumItem(mut i) => {
                let num_variants = i.variants.len();
                i.variants = i.variants.into_iter().filter_map(|x| self.fold_item(x)).collect();
                i.variants_stripped |=
                    num_variants != i.variants.len() || i.variants.iter().any(|f| f.is_stripped());
                EnumItem(i)
            }
            TraitItem(mut i) => {
                i.items = i.items.into_iter().filter_map(|x| self.fold_item(x)).collect();
                TraitItem(i)
            }
            ImplItem(mut i) => {
                //TODO::: rm it
                //let replacer_opt = if let Type::ResolvedPath { path, .. } = &i.for_ {
                //    path.segments.first().map(|segment| format!("${{1}}{}::${{2}}", segment.name))
                //} else {
                //    None
                //};
                let impl_name = if let Type::ResolvedPath { path, .. } = &i.for_ {
                    path.segments.first().map(|segment| segment.name.clone())
                } else {
                    None
                };
                i.items = i
                    .items
                    .into_iter()
                    .filter_map(|mut x| {
                        //TODO::: rm it
                        //if let (Some(_), Some(replacer)) = (&x.name, &replacer_opt) {
                        //    // This whole if block allows to use `Self::<method_name>` in method's doc
                        //    // by replacing
                        //    // "Self::<method_name>" inside documentation
                        //    // with
                        //    // <inner.for_.path.segements>::<method>
                        //    x.attrs.doc_strings = x
                        //        .attrs
                        //        .doc_strings
                        //        .into_iter()
                        //        .map(|doc_frag| {
                        //            if let DocFragment::SugaredDoc(line, span, text) = doc_frag {
                        //                let new_doc_line = SELF_MATCHER
                        //                    .replace_all(text.as_ref(), replacer.as_str())
                        //                    .to_string();
                        //                DocFragment::SugaredDoc(line, span, new_doc_line)
                        //            } else {
                        //                doc_frag
                        //            }
                        //        })
                        //        .collect::<Vec<DocFragment>>();
                        //}
                        x.attrs.impl_name = impl_name.clone();
                        self.fold_item(x)
                    })
                    .collect();
                ImplItem(i)
            }
            VariantItem(i) => {
                let i2 = i.clone(); // this clone is small
                match i.kind {
                    VariantKind::Struct(mut j) => {
                        let num_fields = j.fields.len();
                        j.fields = j.fields.into_iter().filter_map(|x| self.fold_item(x)).collect();
                        j.fields_stripped |= num_fields != j.fields.len()
                            || j.fields.iter().any(|f| f.is_stripped());
                        VariantItem(Variant { kind: VariantKind::Struct(j), ..i2 })
                    }
                    _ => VariantItem(i2),
                }
            }
            x => x,
        }
    }

    /// don't override!
    fn fold_item_recur(&mut self, item: Item) -> Option<Item> {
        let Item { attrs, name, source, visibility, def_id, inner, stability, deprecation } = item;

        let inner = match inner {
            StrippedItem(box i) => StrippedItem(box self.fold_inner_recur(i)),
            _ => self.fold_inner_recur(inner),
        };

        Some(Item { attrs, name, source, inner, visibility, stability, deprecation, def_id })
    }

    fn fold_mod(&mut self, m: Module) -> Module {
        Module {
            is_crate: m.is_crate,
            items: m.items.into_iter().filter_map(|i| self.fold_item(i)).collect(),
        }
    }

    fn fold_crate(&mut self, mut c: Crate) -> Crate {
        c.module = c.module.take().and_then(|module| self.fold_item(module));

        {
            let mut guard = c.external_traits.borrow_mut();
            let external_traits = std::mem::replace(&mut *guard, Default::default());
            *guard = external_traits
                .into_iter()
                .map(|(k, mut v)| {
                    v.items = v.items.into_iter().filter_map(|i| self.fold_item(i)).collect();
                    (k, v)
                })
                .collect();
        }
        c
    }
}
