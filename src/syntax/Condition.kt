package org.setql.server.syntax

class Condition(
    val variable1: Variable,
    val operator: String,
    val variable2: Variable) {

    fun toJSON(): String {
        return "{\"variable1\": ${variable1.toJSON()}, \"operator\": \"$operator\", \"variable2\": ${variable2.toJSON()}}"
    }

    companion object {
        var comparators = arrayOf(">", "<", "=", ">=", "<=", "!=", "/e", "/c")
        var predicates = arrayOf("^", "|", ",", "/v")

        fun getComparator(s: String, i: Int): String {
            return comparators.fold("") { acc, e ->
                //println("$acc, $e, ${i + e.length} <= ${s.length}, ${s.substring(i, i + e.length)} == $e")
                if (i + e.length <= s.length && s.substring(i, i + e.length) == e) {
                    e
                } else {
                    acc
                }
            }
        }

        fun getPredicate(s: String, i: Int): String {
            return predicates.fold("") { acc, e ->
                //println("$acc, $e, ${i + e.length} <= ${s.length}, ${s.substring(i, i + e.length)} == $e")
                if (i + e.length <= s.length && s.substring(i, i + e.length) == e) {
                    e
                } else {
                    acc
                }
            }
        }
    }
}