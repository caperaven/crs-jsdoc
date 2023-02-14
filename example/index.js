/**
 * @function compileExpression - It takes a condition and a process, and returns a function that takes a context, process, and item, and returns the
 * result of evaluating the condition
 * @param condition {String} - The condition to be evaluated.
 * @param process {Object} - the process object
 *
 * @returns A function that takes in a context, process, and item and returns the result of the expression.
 */

/**
 * @function test - This is a test function
 * line2
 * line3
 * @param firstName {String} - The first name of the person
 * @param [lastName="Doe"] {String} - The last name of the person
 * @returns {String} - The full name of the person
 *
 * @example <caption>javascript example</caption>
 * crs.call("person", "test", {
 *     firstName: "John",
 *     lastName: "Doe"
 * });
 *
 * @example <caption>json example</caption>
 * {
 *     "type": "person",
 *     "action": "test",
 *     "args: {
 *         "firstName": "John",
 *         "lastName": "Doe"
 *     }
 * }
 */
function test(firstName, lastName = "Doe") {

}