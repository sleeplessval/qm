
# quickmath (qm)

A small, pretty command line calculator.

Features expression evaluation, variable assignment, an interactive mode, pretty
output, and a variety of preset constants and functions focused on pure mathematics,
data science, and computer science.

More information is available on the [project wiki](https://git.vwolfe.io/valerie/qm/wiki).

## Installation

### Manual Install

<details>
<summary>Release Binary</summary>
Copy the compiled binary from the <a href="https://git.vwolfe.io/valerie/qm/releases">releases page</a>
to a directory in <code>$PATH</code>, such as <code>/usr/bin/</code>.
</details>

<details>
<summary>Compile from Source</summary>
Compile using cargo with the command <code>cargo build --release</code> and copy
the file from <code>target/release/</code> to a directory in <code>$PATH</code>,
such as <code>/usr/bin/</code>.
</details>

### Package Managers

<details>
<summary>Cargo: <code>quickmath</code></summary>
Install the package using Cargo with the command <code>cargo install quickmath</code>.
</details>

## Libraries

- [evalexpr](https://crates.io/crates/evalexpr) — expression evaluator
- [termion](https://crates.io/crates/termion) — ANSI formatting

