# sheetruby

Run Ruby code in Google Sheets using [mruby/edge](https://github.com/mrubyedge/mrubyedge)

## What is sheetruby?

sheetruby embeds the [mruby/edge](https://github.com/mrubyedge/mrubyedge) VM into Google Apps Script, allowing you to execute Ruby code directly in your spreadsheets. It compiles to WebAssembly and runs entirely within Google Sheets' Apps Script environment.

## How to Use

1. Open your Google Spreadsheet
2. Go to **Extensions** → **Apps Script**
3. Copy the contents of `combined.js` and paste it into the script editor
4. Save the script
5. Return to your spreadsheet - the `EVAL_RUBY_SCRIPT()` function is now available!

## `EVAL_RUBY_SCRIPT()` Function Specification

```javascript
EVAL_RUBY_SCRIPT(ruby_code, [arg1], [arg2], [arg3])
```

### Parameters

- **First argument (required)**: Ruby script code
  - Can be a string literal or reference to a cell
- **Arguments 2-4 (optional)**: Data to pass to the Ruby script
  - Can reference cells or ranges
  - Accessible in Ruby as global variables `$arg1`, `$arg2`, `$arg3`
  - Currently supports up to 3 arguments

### Type Handling

- **Single cell**: Type is automatically inferred (number, string, boolean)
- **Multiple cells**: Passed as a 2D array (Array of Arrays)
- **Return value**: Type is automatically inferred from Ruby result via JSON serialization

### Examples

**Basic calculation:**
```ruby
=EVAL_RUBY_SCRIPT("1 + 2")
# => 3
```

**Using cell references:**
```ruby
=EVAL_RUBY_SCRIPT("$arg1 * 2", A1)
# If A1 = 21, returns 42
```

**Array processing:**
```ruby
=EVAL_RUBY_SCRIPT("$arg1.map { |x| x * $arg2 }", A1:A3, B1)
# If A1:A3 = [[1], [2], [3]] and B1 = 10
# Returns [10, 20, 30]
```

**Multiple arguments:**
```ruby
=EVAL_RUBY_SCRIPT("$arg1 + $arg2 + $arg3", A1, B1, C1)
# Sum of three cells
```

**String manipulation:**
```ruby
=EVAL_RUBY_SCRIPT("$arg1.upcase", A1)
# If A1 = "hello", returns "HELLO"
```

**Math functions (with mrubyedge-math):**
```ruby
=EVAL_RUBY_SCRIPT("Math.sqrt($arg1)", 16)
# => 4.0
```

## Building from Source

### Prerequisites

1. Install [Emscripten SDK](https://emscripten.org/docs/getting_started/downloads.html)
   ```bash
   git clone https://github.com/emscripten-core/emsdk.git
   cd emsdk
   ./emsdk install latest
   ./emsdk activate latest
   source ./emsdk_env.sh
   ```

2. Install Rust with the `wasm32-unknown-emscripten` target:
   ```bash
   rustup target add wasm32-unknown-emscripten
   ```

3. Set the `BINDGEN_EXTRA_CLANG_ARGS` environment variable:
   ```bash
   export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$EMSDK/upstream/emscripten/cache/sysroot"
   ```

### Build

```bash
make build
```

This will generate `combined.js` in the project root.

## Contributing

Bug reports and contributions are welcome! Please feel free to:

- Report issues on [GitHub Issues](https://github.com/mrubyedge/sheetruby/issues)
- Submit pull requests
- Share your use cases and feedback

## License

See [LICENSE](LICENSE) file for details.
