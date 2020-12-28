/*
Copyright 2020 Google LLC

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

use lockjaw::{injectable, module, module_impl, root_epilogue};
use printer::Printer;

#[injectable]
pub struct PrinterImpl {}

impl Printer for PrinterImpl {
    fn print(&self, message: &str) {
        println!("{}", message);
    }
}

#[module]
pub struct Module {}

#[module_impl]
impl Module {
    #[binds]
    pub fn bind_printer(impl_: crate::PrinterImpl) -> impl ::printer::Printer {}
}

root_epilogue!();