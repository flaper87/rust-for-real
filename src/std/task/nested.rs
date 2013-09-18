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
 * For more information about tasks' behaviour, please, refer to the README
 * under std/task.
 */

extern mod extra;

use std::task;
use extra::comm::DuplexStream;


fn simple_nesting() {
    do task::spawn {
        let (parent, child) = DuplexStream::<int, int>();

        child.send(0);

        // Using a supervised task to avoid
        // propagating linked failures to the
        // parent task in case it ends before
        // the child does.
        do task::spawn_supervised {
            loop {
                match child.try_recv() {
                    Some(i) => child.send(i + 1),
                    None    => break
                }
            }
        };

        loop {
            if parent.peek() {
                let start = parent.recv();
                println(fmt!("%i", start));

                if start == 10 {
                    println("Counter is now 10");
                    break;
                }

                parent.send(start + 1);
            }
        }
    }
}

fn supervised_task() {
    do task::spawn { 
        do task::spawn_supervised { 
            // This won't make the parent
            // task fail.
            fail!() 
        }
    }
}

fn try_task() {
    // This task fails but won't make the caller
    // task fail, instead, it'll return an error result.
    let failure: Result<(), ()> = do task::try {
        fail!("BAAAAAAD");
    };

    assert!(failure.is_err());

    // This task won't fail and will return a ~str
    let success: Result<~str, ()> = do task::try {
        ~"Yo, you know how to do things"
    };
    assert!(success.is_ok());
    println(success.unwrap());
}


fn main() {
    simple_nesting();
    supervised_task();
    try_task();
}
