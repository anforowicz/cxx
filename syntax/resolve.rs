use crate::syntax::instantiate::NamedImplKey;
use crate::syntax::{ExternFn, Lifetimes, NamedType, Namespace, Pair, Types};
use proc_macro2::Ident;

#[derive(Copy, Clone)]
pub(crate) struct Resolution<'a> {
    pub name: &'a Pair,
    pub generics: &'a Lifetimes,
}

impl<'a> Types<'a> {
    pub(crate) fn resolve(&self, ident: &impl UnresolvedName) -> Resolution<'a> {
        let ident = ident.ident();
        match self.try_resolve(ident) {
            Some(resolution) => resolution,
            None => panic!("Unable to resolve type `{}`", ident),
        }
    }

    pub(crate) fn try_resolve(&self, ident: &impl UnresolvedName) -> Option<Resolution<'a>> {
        let ident = ident.ident();
        self.resolutions.get(ident).copied()
    }
}

pub(crate) trait UnresolvedName {
    fn ident(&self) -> &Ident;
}

impl UnresolvedName for Ident {
    fn ident(&self) -> &Ident {
        self
    }
}

impl UnresolvedName for NamedType {
    fn ident(&self) -> &Ident {
        &self.rust
    }
}

impl<'a> UnresolvedName for NamedImplKey<'a> {
    fn ident(&self) -> &Ident {
        self.rust
    }
}

impl ExternFn {
    pub(crate) fn resolve_namespace<'a>(&'a self, types: &'a Types) -> &'a Namespace {
        self.sig
            .receiver
            .as_ref()
            .map_or(&self.name.namespace, |receiver| {
                &types.resolve(&receiver.ty).name.namespace
            })
    }
}
