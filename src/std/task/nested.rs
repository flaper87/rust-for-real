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

fn main() {
    simple_nesting();
}
