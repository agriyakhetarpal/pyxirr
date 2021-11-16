const MAX_ERROR: f64 = 1e-9;
const MAX_ITERATIONS: u32 = 50;

pub fn find_root_newton_raphson<Func, Deriv>(start: f64, f: Func, d: Deriv) -> f64
where
    Func: Fn(f64) -> f64,
    Deriv: Fn(f64) -> f64,
{
    // x[n + 1] = x[n] - f(x[n])/f'(x[n])

    let mut x = start;

    for _ in 0..MAX_ITERATIONS {
        let res = f(x);

        if res.abs() < MAX_ERROR {
            return x;
        }

        let delta = res / d(x);

        if delta.abs() < MAX_ERROR {
            return x - delta;
        }

        x -= delta;
    }

    f64::NAN
}

pub fn find_root_newton_raphson_with_default_deriv<Func>(start: f64, f: Func) -> f64
where
    Func: Fn(f64) -> f64,
{
    // deriv = (f(x + e) - f(x - e))/((x + e) - x)
    // multiply denominator by 2 for faster convergence

    // https://programmingpraxis.com/2012/01/13/excels-xirr-function/

    find_root_newton_raphson(start, &f, |x: f64| {
        (f(x + MAX_ERROR) - f(x - MAX_ERROR)) / (2.0 * MAX_ERROR)
    })
}

pub fn find_root_newton_raphson_with_brute_force<Func, Deriv>(
    start: f64,
    ranges: &[(f64, f64, f64)],
    f: Func,
    d: Deriv,
) -> f64
where
    Func: Fn(f64) -> f64,
    Deriv: Fn(f64) -> f64,
{
    let is_good_rate = |rate: f64| rate.is_finite() && f(rate).abs() < 1e-3;

    let rate = find_root_newton_raphson(start, &f, &d);

    if is_good_rate(rate) {
        return rate;
    }

    for (min, max, step) in ranges.into_iter() {
        let mut guess = *min;
        while guess < *max {
            let rate = find_root_newton_raphson(guess, &f, &d);
            if is_good_rate(rate) {
                return rate;
            }
            guess += step;
        }
    }

    f64::NAN
}
