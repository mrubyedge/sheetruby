/**
 * eval ruby script and return integer value
 * @param {string} input ruby code.
 * @return The integer result of the Ruby script.
 * @customfunction
*/
function EVAL_RUBY_SCRIPT_INT(text) {
    return Module.ccall(
        'eval_ruby_script_int',
        'number',
        ['string'],
        [text]
    );
}

/**
 * eval ruby script and return float value
 * @param {string} input ruby code.
 * @return The float result of the Ruby script.
 * @customfunction
*/
function EVAL_RUBY_SCRIPT_FLOAT(text) {
    return Module.ccall(
        'eval_ruby_script_float',
        'number',
        ['string'],
        [text]
    );
}

/**
 * eval ruby script and return boolean value
 * @param {string} input ruby code.
 * @return The boolean result of the Ruby script.
 * @customfunction
*/
function EVAL_RUBY_SCRIPT_BOOL(text) {
    var result = Module.ccall(
        'eval_ruby_script_bool',
        'number',
        ['string'],
        [text]
    );
    return result !== 0;
}

/**
 * eval ruby script and return string value
 * @param {string} input ruby code.
 * @return The string result of the Ruby script.
 * @customfunction
*/
function EVAL_RUBY_SCRIPT_STRING(text) {
    var ptr = Module.ccall(
        'eval_ruby_script_string',
        'number',
        ['string'],
        [text]
    );
    if (ptr === 0) {
        return "";
    }
    return Module.UTF8ToString(ptr);
}
