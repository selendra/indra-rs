// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

use crate::utils;
use heck::{CamelCase, SnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use synstructure::Structure;

pub fn event(s: Structure) -> TokenStream {
    let subxt = utils::use_crate("substrate-subxt");
    let codec = utils::use_crate("parity-scale-codec");
    let ident = &s.ast().ident;
    let generics = &s.ast().generics;
    let module = utils::module_name(generics);
    let event_name = utils::ident_to_name(ident, "Event").to_camel_case();
    let event = format_ident!("{}", event_name.to_snake_case());
    let event_trait = format_ident!("{}EventExt", event_name);

    quote! {
        impl<T: #module> #subxt::Event<T> for #ident<T> {
            const MODULE: &'static str = MODULE;
            const EVENT: &'static str = #event_name;
        }

        /// Event extension trait.
        pub trait #event_trait<T: #module> {
            /// Retrieves the event.
            fn #event(&self) -> Result<Option<#ident<T>>, #codec::Error>;
        }

        impl<T: #module> #event_trait<T> for #subxt::ExtrinsicSuccess<T> {
            fn #event(&self) -> Result<Option<#ident<T>>, #codec::Error> {
                self.find_event()
            }
        }
    }
}
