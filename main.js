/**
 * eval ruby script and return integer value
 * Arguments are accessible as $arg1, $arg2, $arg3 in Ruby code.
 * @param {string} text ruby code.
 * @param {...(string|number|Array<Array<string|number>>)} args arguments accessible as $arg1, $arg2, $arg3 (up to 3 arguments).
 * @return The result of the Ruby script (type inferred from JSON).
 * @customfunction
*/
function EVAL_RUBY_SCRIPT(text) {
    var args = Array.prototype.slice.call(arguments, 1);
    var result;
    if (args.length === 0) {
        result = Module.ccall(
            'eval_ruby_script_returning_json',
            'string',
            ['string'],
            [text]
        );
    } else if (args.length === 1) {
        result = Module.ccall(
            'eval_ruby_script_returning_json1',
            'string',
            ['string', 'string'],
            [text, JSON.stringify(args[0])]
        );
    } else if (args.length === 2) {
        result = Module.ccall(
            'eval_ruby_script_returning_json2',
            'string',
            ['string', 'string', 'string'],
            [text, JSON.stringify(args[0]), JSON.stringify(args[1])]
        );
    } else {
        // 3 or more arguments - use json3 (max 3 args supported)
        result = Module.ccall(
            'eval_ruby_script_returning_json3',
            'string',
            ['string', 'string', 'string', 'string'],
            [text, JSON.stringify(args[0]), JSON.stringify(args[1]), JSON.stringify(args[2])]
        );
    }
    return JSON.parse(result);
}