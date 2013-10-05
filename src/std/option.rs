// Copyright 2013 The rust-for-real developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/**
 *
 * Options represent optional values that can be either Some(T) or None
 *
 * Optional values are very useful when working with non-deterministic
 * environments, concurrent environments, signals / events, etc.
 *
 * A common way to unwrap `Option` values is to use match and checking
 * whether it holds a value or not. Although this is widely used, `Option`
 * also implements a big set of useful methods that cover most of the cases:
 *
 *  - unwrap: Returns the value or fails if it's None.
 *
 *  - unwrap_or: This returns the held value if any or the value passed to
 *  this call.
 *
 *  - is_(none|some): Return a boolean saying whether it holds a value or None.
 *
 */

fn option_match() {
    let value: Option<int> = Some(10);

    // This is the most common
    // way to use Optional values.
    match value {
        // Using `_` as variable name since it's not being used,
        // this silents 'unused variable' warnings from the compiler
        Some(_) => println("It is indeed an int and has a value"),
        None => println("It'll never get here")
    }
}

fn optional_arguments(some_str: Option<&str>) {

    // We could invert this by
    // checking for `is_some`
    // instead of `is_none`
    if (some_str.is_none()) {
        println("Got nothing");
    } else {
        println(fmt!("Got %s", some_str.unwrap()));
    }

    // A simpler way to do this would be
    println(fmt!("Got %s", some_str.unwrap_or("nothing")));
}

fn main() {
    option_match();

    optional_arguments(Some("Testing"));
    optional_arguments(None);
}
