package org.setql.operations

import org.setql.server.generator.*
import org.setql.server.syntax.Condition
import org.setql.server.syntax.Variable
import org.setql.server.syntax.Function
import org.setql.server.syntax.Set
import org.setql.server.syntax.Condition.Companion.getPredicate
import org.setql.server.syntax.Condition.Companion.getComparator

fun parseSet(s: String, i_: Int): Pair<Set, Int> {
    var i = i_

    while (i < s.length && s[i] == ' ') {
        i += 1
    }

    var brackets = 0
    var set = Set()
    while (i < s.length) {
        var comp = getComparator(s, i)
        // if valid as an comp
        if (brackets == 0 && comp.length > 0) {
            var pair = parseCondition(s, i_ + 1)
            set.memberCondition = pair.first
            i = pair.second
            break
        }

        var pred = getPredicate(s, i)
        if (brackets == 0 && pred.length > 0) {
            var pair = parseVariable(s, i_ + 1)
            set.memberVariable = pair.first
            i = pair.second
            break
        }

        if (i < s.length && s[i] == '(') {
            brackets += 1
        }
        if (i < s.length && s[i] == ')') {
            if (brackets == 0) {
                break
            }
            brackets -= 1
        }

        if (i < s.length && brackets == 0 && s[i] == '}') {
            break
        }

        i += 1
    }

    while (i < s.length && s[i] == ' ') {
        i += 1
    }

    i += getPredicate(s, i).length

    var pair = parseConditions(s, i)
    set.conditions = pair.first
    i = pair.second

    i += 1

    return Pair(set, i)
}

fun parseVariable(s: String, i_: Int): Pair<Variable, Int> {
    var i = i_

    while (i < s.length && s[i] == ' ') {
        i += 1
    }

    if (s[i] == '{') {
        var pair = parseSet(s, i)
        i = pair.second

        while (i < s.length && s[i] == ' ') {
            i += 1
        }

        //i += getPredicate(s, i).length
        return Pair(pair.first, i)
    }

    var brackets = 0
    while (i < s.length) {
        // function call is a bracket but not a subscript
        if (s[i] == '(' && (i == 0 || s[i - 1] != '_')) {
            var func = s.substring(i_, i).trim()
            var params: Array<Variable> = arrayOf()
            i += 1
            while (i < s.length) {
                if (s[i] == ')') {
                    i += 1
                    break
                }

                var pair = parseVariable(s, i)
                params += pair.first
                i = pair.second
            }
            return Pair(Function(func, params), i)
        }

        var comp = getComparator(s, i)
        // if valid as an comp, ',' case is covered here as well though it could also be in a parameter list context
        if (brackets == 0 && comp.length > 0) {
            return Pair(Variable(s.substring(i_, i).trim()), i)
        }

        var pred = getPredicate(s, i)
        if (brackets == 0 && pred.length > 0) {
            if (pred == "|" && s.substring(i_, i).trim() == "") {
                i += 1
                var pair = parseVariable(s, i)
                i = pair.second + 1
                return Pair(Function("abs", arrayOf(pair.first)), i)
            }

            return Pair(Variable(s.substring(i_, i).trim()), i)
        }

        if (i < s.length && s[i] == '(') {
            brackets += 1
        }
        if (i < s.length && s[i] == ')') {
            if (brackets == 0) {
                break
            }
            brackets -= 1
        }

        if (i < s.length && s[i] == '}') {
            break
        }

        i += 1
    }

    return Pair(Variable(s.substring(i_, i).trim()), i)
}

fun parseCondition(s: String, i_: Int): Pair<Condition, Int> {
    var i = i_

    while (i < s.length && s[i] == ' ') {
        i += 1
    }

    var pair = parseVariable(s, i)
    var vars: Array<Variable> = arrayOf(pair.first)
    i = pair.second

    while (i < s.length && s[i] == ' ') {
        i += 1
    }

    var brackets = 0
    var comp = ""
    while (i < s.length) {
        comp = getComparator(s, i)
        // if valid as an comp
        if (brackets == 0 && comp.length > 0) {
            i += comp.length;
            pair = parseVariable(s, i)
            vars += pair.first
            i = pair.second
            break
        }

        var pred = getPredicate(s, i)
        if (brackets == 0 && pred.length > 0) {
            i + 1
            break
        }

        if (i < s.length && s[i] == '(') {
            if (brackets == 0) {
                break
            }
            brackets += 1
        }
        if (i < s.length && s[i] == ')') {
            brackets -= 1
        }

        i += 1
    }

    if (vars.size < 2 || comp == "") {
        throw Error("No condition found, found vars ${vars.map{e -> e.toJSON()}.joinToString()} and comp $comp")
    }

    return Pair(Condition(vars[0], comp, vars[1]), i)
}

fun parseConditions(s: String, i_: Int): Pair<Array<Condition>, Int> {
    var i = i_

    while (i < s.length && s[i] == ' ') {
        i += 1
    }

    var conditions: Array<Condition> = arrayOf()
    while (i < s.length) {
        if (i < s.length && (s[i] == ')' || s[i] == '}')) {
            break
        }

        var cond = parseCondition(s, i)
        i = cond.second
        conditions += cond.first

        while (i < s.length && s[i] == ' ') {
            i += 1
        }

        i += getPredicate(s, i).length
    }

    return Pair(conditions, i)
}

fun sqlFromSetQLQuery(input: String): String {
    var conds = parseConditions(input, 0).first
    var sql = generateMySql(conds)

    return sql
}