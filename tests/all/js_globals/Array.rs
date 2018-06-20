#![allow(non_snake_case)]

use project;

#[test]
fn index_of() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_index_of(this: &js::Array, value: JsValue, from_index: i32) -> i32 {
                this.index_of(value, from_index)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "c", "x", "n"];
                let index = wasm.get_index_of(characters, "x", 0);
                let notFoundIndex = wasm.get_index_of(characters, "z", 0);

                assert.equal(index, 2);
                assert.equal(notFoundIndex, -1);

                let withFromIndex = wasm.get_index_of(characters, "x", -3);
                let withFromIndexNotFound = wasm.get_index_of(characters, "a", -2);

                assert.equal(withFromIndex, 2);
                assert.equal(withFromIndexNotFound, -1);
            }
        "#)
        .test()
}

#[test]
fn last_index_of() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_last_index_of(this: &js::Array, value: JsValue, from_index: i32) -> i32 {
                this.last_index_of(value, from_index)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "x", "c", "x", "n"];
                let index = wasm.get_last_index_of(characters, "x", 5);
                let notFoundIndex = wasm.get_last_index_of(characters, "z", 5);

                assert.equal(index, 3);
                assert.equal(notFoundIndex, -1);

                let withFromIndex = wasm.get_last_index_of(characters, "x", 2);
                let withFromIndexNotFound = wasm.get_last_index_of(characters, "x", 0);

                assert.equal(withFromIndex, 1);
                assert.equal(withFromIndexNotFound, -1);
            }
        "#)
        .test()
}

#[test]
fn join() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn join_array(this: &js::Array, delimiter: &str) -> String {
                this.join(delimiter)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "c", "x", "n"];
                let stringValue = wasm.join_array(characters, ", ");

                assert.equal("a, c, x, n", stringValue);
                let withForwardSlash = wasm.join_array(characters, "/");
                assert.equal("a/c/x/n", withForwardSlash);
            }
        "#)
        .test()
}

#[test]
fn slice() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn create_slice(this: &js::Array, start: u32, end: u32) -> js::Array {
                this.slice(start, end)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "c", "x", "n", 1, "8"];
                let subset = wasm.create_slice(characters, 1, 3);

                assert.equal(subset[0], "c");
                assert.equal(subset[1], "x");
            }
        "#)
        .test()
}

#[test]
fn fill() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn fill_with(this: &js::Array, value: JsValue, start: u32, end: u32) -> js::Array {
                this.fill(value, start, end)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "c", "x", "n", 1, "8"];
                let subset = wasm.fill_with(characters, 0, 0, 3);

                assert.equal(subset[0], 0);
                assert.equal(subset[4], 1);
            }
        "#)
        .test()
}

#[test]
fn copy_within() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn copy_values_within_array(this: &js::Array, target: i32, start: i32, end: i32) -> js::Array {
                this.copy_within(target, start, end)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                wasm.copy_values_within_array(characters, 1, 4, 5);

                assert.equal(characters[1], 1);

                // if negatives were used
                wasm.copy_values_within_array(characters, -1, -3, -2);
                assert.equal(characters[5], 3);
            }
        "#)
        .test()
}

#[test]
fn pop() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn pop_in_it(this: &js::Array) -> JsValue {
                this.pop()
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let item = wasm.pop_in_it(characters);
                assert.equal(item, 2);
                assert.equal(characters.length, 5);
            }
        "#)
        .test()
}


#[test]
fn push() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn push_it_along(this: &js::Array, value: JsValue) -> u32 {
                this.push(value)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let length = wasm.push_it_along(characters, "a");
                assert.equal(length, 7);
                assert.equal(characters[6], "a");
            }
        "#)
        .test()
}

#[test]
fn reverse() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn reverse_array(this: &js::Array) -> js::Array {
                this.reverse()
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let reversed = wasm.reverse_array(characters);
                assert.equal(reversed[0], 2);
                assert.equal(reversed[5], 8);
            }
        "#)
        .test()
}