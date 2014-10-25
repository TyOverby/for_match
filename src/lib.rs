#![feature(macro_rules)]

macro_rules! for_match (
    ($e: expr $($($b: pat)|+ $(if $cond: expr)* => $a: expr),+) => {
        {
            let mut iter = $e;
            loop {
                let x = match iter.next() {
                    Some(x) => x,
                    None => {break;}
                };
                match x {
                    $($($b)|+ $(if $cond)* => $a),+
                };
            }
        }
    };
)
