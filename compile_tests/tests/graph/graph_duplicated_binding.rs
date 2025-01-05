/*
Copyright 2021 Google LLC

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
extern crate lockjaw;

use lockjaw::{builder_modules, component, injectable, module, qualifier, subcomponent, Cl};

lockjaw::prologue!(
    "../../../compile_tests/tests/graph/graph_duplicated_binding.rs",
    "",
    "test"
);
struct Foo {}

#[injectable]
impl Foo {
    #[inject]
    pub fn new() -> Self {
        Self {}
    }
}

struct M {}

#[module]
impl M {
    #[provides]
    pub fn provide_foo() -> crate::Foo {
        Foo {}
    }
}

#[component(modules: M)]
trait S {
    fn foo() -> crate::Foo;
}

fn main() {}

lockjaw::epilogue!(test);