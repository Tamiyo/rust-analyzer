//! This module defines an accumulator for completions which are going to be presented to user.

pub(crate) mod attribute;
pub(crate) mod dot;
pub(crate) mod record;
pub(crate) mod pattern;
pub(crate) mod fn_param;
pub(crate) mod keyword;
pub(crate) mod snippet;
pub(crate) mod qualified_path;
pub(crate) mod unqualified_path;
pub(crate) mod postfix;
pub(crate) mod macro_in_item_position;
pub(crate) mod trait_impl;
pub(crate) mod mod_;

use hir::{ModPath, ScopeDef, Type};

use crate::{
    item::Builder,
    render::{
        const_::render_const,
        enum_variant::render_variant,
        function::render_fn,
        macro_::render_macro,
        pattern::{render_struct_pat, render_variant_pat},
        render_field, render_resolution, render_tuple_field,
        type_alias::render_type_alias,
        RenderContext,
    },
    CompletionContext, CompletionItem,
};

/// Represents an in-progress set of completions being built.
#[derive(Debug, Default)]
pub struct Completions {
    buf: Vec<CompletionItem>,
}

impl Into<Vec<CompletionItem>> for Completions {
    fn into(self) -> Vec<CompletionItem> {
        self.buf
    }
}

impl Builder {
    /// Convenience method, which allows to add a freshly created completion into accumulator
    /// without binding it to the variable.
    pub(crate) fn add_to(self, acc: &mut Completions) {
        acc.add(self.build())
    }
}

impl Completions {
    pub(crate) fn add(&mut self, item: CompletionItem) {
        self.buf.push(item.into())
    }

    pub(crate) fn add_all<I>(&mut self, items: I)
    where
        I: IntoIterator,
        I::Item: Into<CompletionItem>,
    {
        items.into_iter().for_each(|item| self.add(item.into()))
    }

    pub(crate) fn add_field(&mut self, ctx: &CompletionContext, field: hir::Field, ty: &Type) {
        let item = render_field(RenderContext::new(ctx), field, ty);
        self.add(item);
    }

    pub(crate) fn add_tuple_field(&mut self, ctx: &CompletionContext, field: usize, ty: &Type) {
        let item = render_tuple_field(RenderContext::new(ctx), field, ty);
        self.add(item);
    }

    pub(crate) fn add_resolution(
        &mut self,
        ctx: &CompletionContext,
        local_name: String,
        resolution: &ScopeDef,
    ) {
        if let Some(item) = render_resolution(RenderContext::new(ctx), local_name, resolution) {
            self.add(item);
        }
    }

    pub(crate) fn add_macro(
        &mut self,
        ctx: &CompletionContext,
        name: Option<String>,
        macro_: hir::MacroDef,
    ) {
        let name = match name {
            Some(it) => it,
            None => return,
        };
        if let Some(item) = render_macro(RenderContext::new(ctx), None, name, macro_) {
            self.add(item);
        }
    }

    pub(crate) fn add_function(
        &mut self,
        ctx: &CompletionContext,
        func: hir::Function,
        local_name: Option<String>,
    ) {
        let item = render_fn(RenderContext::new(ctx), None, local_name, func);
        self.add(item)
    }

    pub(crate) fn add_variant_pat(
        &mut self,
        ctx: &CompletionContext,
        variant: hir::Variant,
        local_name: Option<hir::Name>,
    ) {
        if let Some(item) = render_variant_pat(RenderContext::new(ctx), variant, local_name) {
            self.add(item);
        }
    }

    pub(crate) fn add_struct_pat(
        &mut self,
        ctx: &CompletionContext,
        strukt: hir::Struct,
        local_name: Option<hir::Name>,
    ) {
        if let Some(item) = render_struct_pat(RenderContext::new(ctx), strukt, local_name) {
            self.add(item);
        }
    }

    pub(crate) fn add_const(&mut self, ctx: &CompletionContext, constant: hir::Const) {
        if let Some(item) = render_const(RenderContext::new(ctx), constant) {
            self.add(item);
        }
    }

    pub(crate) fn add_type_alias(&mut self, ctx: &CompletionContext, type_alias: hir::TypeAlias) {
        if let Some(item) = render_type_alias(RenderContext::new(ctx), type_alias) {
            self.add(item)
        }
    }

    pub(crate) fn add_qualified_enum_variant(
        &mut self,
        ctx: &CompletionContext,
        variant: hir::Variant,
        path: ModPath,
    ) {
        let item = render_variant(RenderContext::new(ctx), None, None, variant, Some(path));
        self.add(item);
    }

    pub(crate) fn add_enum_variant(
        &mut self,
        ctx: &CompletionContext,
        variant: hir::Variant,
        local_name: Option<String>,
    ) {
        let item = render_variant(RenderContext::new(ctx), None, local_name, variant, None);
        self.add(item);
    }
}
