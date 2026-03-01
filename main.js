/**
 * eval ruby script and return integer value
 * Arguments are accessible as $arg1, $arg2, etc. in Ruby code.
 * @param {string} text ruby code.
 * @param {...(string|number|Array<Array<string|number>>)} args arguments accessible as $arg1, $arg2, etc. (converted to string).
 * @return The integer result of the Ruby script.
 * @customfunction
*/
function EVAL_RUBY_SCRIPT(text) {
    var args = Array.prototype.slice.call(arguments, 1);
    if (args.length === 0) {
        const result = Module.ccall(
            'eval_ruby_script_returning_json',
            'string',
            ['string'],
            [text]
        );
        return JSON.parse(result);
    } else if (args.length === 1) {
        const result = Module.ccall(
            'eval_ruby_script_returning_json1',
            'string',
            ['string', 'string'],
            [text, JSON.stringify(args[0])]
        );
        return JSON.parse(result);
    } else {
        // For now, only support up to 1 argument
        // TODO: Add support for more arguments
        const result = Module.ccall(
            'eval_ruby_script_returning_json1',
            'string',
            ['string', 'string'],
            [text, JSON.stringify(args[0])]
        );
        return JSON.parse(result);
    }
}