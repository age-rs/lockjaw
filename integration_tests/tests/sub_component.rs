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
use lockjaw::{component, module, subcomponent, Cl};

struct SubcomponentModule {}

#[module]
impl SubcomponentModule {
    #[provides]
    pub fn provide_i32() -> i32 {
        32
    }
}

#[subcomponent(modules: [SubcomponentModule])]
pub trait MySubcomponent<'a> {
    fn fi64(&self) -> i64;
    fn fi32(&self) -> i32;
}

struct ParentComponentModule {}

// ANCHOR: list
#[module(subcomponents: [MySubcomponent])]
impl ParentComponentModule {
    // ANCHOR_END: list
    #[provides]
    pub fn provide_i64() -> i64 {
        64
    }
}

#[component(modules: [ParentComponentModule])]
pub trait MyComponent {
    fn sub(&'_ self) -> Cl<dyn MySubcomponentBuilder<'_>>;
}

#[test]
pub fn parent_binding() {
    let component: Box<dyn MyComponent> = <dyn MyComponent>::new();
    let sub: Cl<dyn MySubcomponent> = component.sub().build();

    assert_eq!(sub.fi64(), 64);
}

#[test]
pub fn sub_binding() {
    let component: Box<dyn MyComponent> = <dyn MyComponent>::new();
    let sub: Cl<dyn MySubcomponent> = component.sub().build();

    assert_eq!(sub.fi32(), 32);
}

lockjaw::epilogue!();
