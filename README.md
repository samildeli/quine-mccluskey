<h1>Quine-McCluskey</h1>

[![crates.io](https://samildeli.com/images/crates.io.png)](https://crates.io/crates/quine-mccluskey)
[![docs.rs](https://samildeli.com/images/docs.rs.png)](https://docs.rs/quine-mccluskey)

Boolean function minimizer based on [Quine-McCluskey algorithm](https://en.wikipedia.org/wiki/Quine%E2%80%93McCluskey_algorithm).

## Example

Given the boolean function expressed by the following truth table:

|  A  |  B  |  C  | Output |
| :-: | :-: | :-: | :----: |
|  0  |  0  |  0  |   1    |
|  0  |  0  |  1  |   0    |
|  0  |  1  |  0  |   X    |
|  0  |  1  |  1  |   0    |
|  1  |  0  |  0  |   0    |
|  1  |  0  |  1  |   1    |
|  1  |  1  |  0  |   0    |
|  1  |  1  |  1  |   X    |

We can minimize it in Sum of Products form using `minimize` with minterms and maxterms and `Form::SOP`:

```rust
use quine_mccluskey as qmc;
use quine_mccluskey::MinimizeTimeoutStrategy as mts;

let minimize_options = qmc::MinimizeOptions::default()
    .set_find_all_solutions(false)
    .set_timeout_strategy(mts::Indefinitely);

let mut solutions = qmc::minimize_ex(
    &qmc::DEFAULT_VARIABLES[..3],
    &[0, 5],        // minterms
    &[1, 3, 4, 6],  // maxterms
    qmc::SOP,
    minimize_options,
)
.unwrap();

assert_eq!(
    solutions.pop().unwrap().to_string(),
    "(A ∧ C) ∨ (~A ∧ ~C)"
);
```

or using `minimize_minterms` with minterms and don't cares:

```rust
use quine_mccluskey as qmc;
use quine_mccluskey::MinimizeTimeoutStrategy as mts;

let minimize_options = qmc::MinimizeOptions::default()
    .set_find_all_solutions(false)
    .set_timeout_strategy(mts::Indefinitely);

let mut solutions = qmc::minimize_minterms_ex(
    &qmc::DEFAULT_VARIABLES[..3],
    &[0, 5],  // minterms
    &[2, 7],  // don't cares
    minimize_options,
)
.unwrap();

assert_eq!(
    solutions.pop().unwrap().to_string(),
    "(A ∧ C) ∨ (~A ∧ ~C)"
);
```

And in Product of Sums form using `minimize` with minterms and maxterms and `Form::POS`:

```rust
use quine_mccluskey as qmc;
use quine_mccluskey::MinimizeTimeoutStrategy as mts;

let minimize_options = qmc::MinimizeOptions::default()
    .set_find_all_solutions(false)
    .set_timeout_strategy(mts::Indefinitely);

let mut solutions = qmc::minimize_ex(
    &qmc::DEFAULT_VARIABLES[..3],
    &[0, 5],        // minterms
    &[1, 3, 4, 6],  // maxterms
    qmc::POS,
    minimize_options,
)
.unwrap();

assert_eq!(
    solutions.pop().unwrap().to_string(),
    "(A ∨ ~C) ∧ (~A ∨ C)"
);
```

or using `minimize_maxterms` with maxterms and don't cares:

```rust
use quine_mccluskey as qmc;
use quine_mccluskey::MinimizeTimeoutStrategy as mts;

let minimize_options = qmc::MinimizeOptions::default()
    .set_find_all_solutions(false)
    .set_timeout_strategy(mts::Indefinitely);

let mut solutions = qmc::minimize_maxterms_ex(
    &qmc::DEFAULT_VARIABLES[..3],
    &[1, 3, 4, 6],  // maxterms
    &[2, 7],        // don't cares
    minimize_options,
)
.unwrap();

assert_eq!(
    solutions.pop().unwrap().to_string(),
    "(A ∨ ~C) ∧ (~A ∨ C)"
);
```

## Feature flags

- `serde` – Derives the `Serialize` and `Deserialize` traits for structs and enums.
