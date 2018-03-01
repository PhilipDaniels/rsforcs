# rsforcs
A guide to Rust for C# programmers

As of 2018-02-12, builds using Gutenberg v0.3.0, the latest version
available at that date.


## TODO - Content
- [X] Define major sections

## TODO - Theming / Site Infrastructure
- [ ] Improve the structure of the SASS, introduce a colors section.
- [ ] Google Analytics
- [ ] Left/right code blocks
- [ ] Code blocks to have links to the playground?
- [ ] A COPY button on code blocks
- [ ] Use https://github.com/mattico/elasticlunr-rs and add a search box.
- [ ] Code files to be separate from the markdown? See https://github.com/verpeteren/gutenberg-materialize#options-index-page
- [ ] Very nice code samples: https://www.fpcomplete.com/blog/2017/07/iterators-streams-rust-haskell


## Pending material
#### Build
This doesn't handle code coverage but I always do 'watchexec -e rs "cargo test"'.
Actually add some filter examples to the LINQ sections.

#### Performance
[profile.release]
lto = true
codegen-units = 1

[profile.bench]
lto = true
codegen-units = 1

[build]
rustflags = ["-C",  "target-cpu=native"]

#### Syntax
Syntax
iter: I           - immutable moved argument
mut iter: I     - mutable moved argument


#### Blog Improvements
Side By Side Code Blocks

<section class="container">
    <div class="left">
      <div class="code">
        <p>
        Some code
        </p>
        <p>
        Some more code
        </p>
      </div>
    </div>

    <div class="right">
      <div class="code">
        <p>
        Some right-hand-side code
        </p>
        <p>
        Some more right-hand-side code
        </p>
        <p>
        Yet more code
        </p>
      </div>
    </div>
</section>

.container {
  display: flex;
  flex-wrap: wrap;
  align-items: stretch;
}
.left {
  width: 50%;
  order: 0;
}
.right {
  width: 50%;
  order: 1;
}
.code {
  background-color: grey;
  margin: 5px;
  padding: 5px;
  border: 1px solid black;
}

