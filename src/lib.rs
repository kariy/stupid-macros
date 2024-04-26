/// Repeat a function a given number of times.
#[macro_export]
macro_rules! repeat {
    ($func:expr) => {
        while true {
            $func;
        }
    };
    ($func:expr, $count:expr) => {
        for _ in 0..$count {
            $func;
        }
    };
}

/// Time a function and print the time taken for the function execution.
#[macro_export]
macro_rules! timed {
    ($name:ident, $func:expr) => {{
        let $name = {
            let start_time = std::time::Instant::now();
            let result = $func;
            let elapsed = start_time.elapsed();
            println!("{}: {:?}", stringify!($name), elapsed);
            result
        };
        $name
    }};

    ($func:expr) => {{
        let start_time = std::time::Instant::now();
        let result = $func;
        let elapsed = start_time.elapsed();
        println!("{:?}", elapsed);
        result
    }};
}
