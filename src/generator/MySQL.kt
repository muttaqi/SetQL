package org.setql.server.generator

import org.setql.server.syntax.*
import org.setql.server.syntax.Function
import org.setql.server.syntax.Set

fun getFields(setql: Array<Condition>): String {
    var fields: Array<String> = arrayOf()
    for (cond in setql) {
        if (cond.variable1.label == "/Q" && arrayOf("/c", "=").contains(cond.operator)) {
            var set = cond.variable2
            if (set is Set) {
                var memberLabel: String?
                if  (set.memberVariable != null) {
                    memberLabel = set.memberVariable?.label
                } else {
                    if (set.memberCondition?.variable2?.label != null && set.memberCondition?.operator == "/e") {
                        return "*"
                    }

                    memberLabel = set.memberCondition?.variable1?.label
                }
                println("MSQL 24 " + memberLabel)

                for (setCond in set.conditions) {
                    var func = setCond.variable1
                    if (func is Function && func.parameters.size == 1 && func.parameters[0].label == memberLabel) {

                        var value = setCond.variable2
                        if (value is Function) {
                            if (Function.fieldAggregators.contains(value.label) && value.parameters.size == 1) {
                                var aggregatedSet = value.parameters[0]

                                if (aggregatedSet is Set && aggregatedSet.memberVariable != null) {
                                    var aggregatedVariable = aggregatedSet.memberVariable
                                    fields += "${Function.fieldAggregatorToSQL[value.label]!!}(${aggregatedVariable?.label!!}) AS ${func.label}"
                                    continue
                                }
                            }

                            if (Function.setAggregators.contains(value.label) && value.parameters.size == 1) {
                                var aggregatedSet = value.parameters[0]

                                if (aggregatedSet is Set && aggregatedSet.memberCondition != null) {
                                    var aggregatedMember = aggregatedSet.memberCondition
                                    fields += "${Function.setAggregatorToSQL[value.label]!!}(*) AS ${func.label}"
                                    continue
                                }
                            }
                        }

                        fields += func.label!!
                    }
                }
            }
        }
    }
    return fields.joinToString()
}

fun getSet(setql: Array<Condition>): String {
    for (cond in setql) {
        if (cond.variable1.label == "/Q" && arrayOf("/c", "=").contains(cond.operator)) {
            var set = cond.variable2
            if (set is Set) {
                if (set.memberCondition != null) {
                    if (set.memberCondition?.variable2?.label != null && set.memberCondition?.operator == "/e") {
                        return set.memberCondition?.variable2?.label!!
                    }
                }

                for (setCond in set.conditions) {
                    if (setCond.variable2?.label != null && setCond.operator == "/e") {
                        return setCond.variable2?.label!!
                    }

                    if (setCond.variable2 is Function && setCond.operator == "=") {
                        var aggregatedSet = setCond.variable2.parameters[0]
                        if (aggregatedSet is Set) {
                            if (aggregatedSet.memberCondition != null) {
                                return aggregatedSet.memberCondition?.variable2?.label!!
                            }
                            for (aggregatedSetCond in aggregatedSet.conditions) {
                                if (aggregatedSetCond.operator == "/e") {
                                    return aggregatedSetCond.variable2.label!!
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    throw Error("No set found")
}

fun getGroupBy(setql: Array<Condition>): String {
    for (cond in setql) {
        if (cond.variable1.label == "/Q" && arrayOf("/c", "=").contains(cond.operator)) {
            var set = cond.variable2
            if (set is Set) {
                if (set.memberCondition != null) {
                    if (set.memberCondition?.variable2?.label != null && set.memberCondition?.operator == "/e") {
                        return ""
                    }
                }

                var setMember = set.memberVariable!!

                for (setCond in set.conditions) {
                    if (setCond.variable2?.label != null && setCond.operator == "/e") {
                        return ""
                    }

                    if (setCond.variable2 is Function && setCond.operator == "=") {
                        var aggregatedSet = setCond.variable2.parameters[0]
                        if (aggregatedSet is Set) {
                            for (aggregatedSetCond in aggregatedSet.conditions) {
                                if (aggregatedSetCond.operator == "=") {
                                    if (aggregatedSetCond.variable1 is Function && aggregatedSetCond.variable2 is Function) {
                                        var fields = arrayOf<String>(aggregatedSetCond.variable1.label!!, aggregatedSetCond.variable2.label!!)
                                        var params = aggregatedSetCond.variable1.parameters.map { e -> e.label } + aggregatedSetCond.variable2.parameters.map { e -> e.label }

                                        if (params.contains(setMember.label) && fields[0] == fields[1]) {
                                            return "GROUP BY ${fields[0]}\n"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return ""
}

fun getConditionsFromSet(set: Set): Array<String> {
    var sqlConds = arrayOf<String>()
    for (cond in set.conditions) {
        var sqlCond = cond.toSQL()
        if (sqlCond != "") {
            sqlConds += sqlCond
        }

        if (cond.variable2 is Function && cond.variable2.parameters.size == 1) {
            var condSet = cond.variable2.parameters[0]
            if (condSet is Set) {
                sqlConds += getConditionsFromSet(condSet)
            }
        }
    }
    return sqlConds
}

fun getConditions(setql: Array<Condition>): String {
    var sqlConds = arrayOf<String>()
    for (cond in setql) {
        if (cond.variable1.label == "/Q" && arrayOf("/c", "=").contains(cond.operator)) {
            var set = cond.variable2
            if (set is Set) {
                sqlConds += getConditionsFromSet(set)
            }
        }
    }
    if (sqlConds.size == 0) {
        return ""
    }

    return "WHERE ${sqlConds.joinToString(" AND ")}\n"
}

fun getSortByAndOrder(setql: Array<Condition>): String {
    for (cond in setql) {
        if (cond.variable1.label == "/Q" && arrayOf("/c", "=").contains(cond.operator)) {
            var set = cond.variable2
            if (set is Set) {
                var member: Variable?
                if (set.memberCondition != null) {
                    member = set.memberCondition?.variable1
                } else {
                    member = set.memberVariable
                }

                if (member?.label!!.contains('_')) {
                    var index = member?.label!!.split("_")[1]
                    var label = member?.label!!.split("_")[0]

                    for (cond in set.conditions) {
                        if (cond.variable1 is Function && cond.variable2 is Function) {
                            if (cond.variable1.label == cond.variable2.label) {
                                var field = cond.variable2.label!!
                                if (cond.variable1.parameters.size == 1 && cond.variable2.parameters.size == 1) {
                                    if (cond.variable1.parameters[0].label!!.contains('_') && cond.variable2.parameters[0].label!!.contains(
                                            '_'
                                        )
                                    ) {
                                        var param1 = cond.variable1.parameters[0].label!!
                                        var param2 = cond.variable2.parameters[0].label!!
                                        if (param1.split("_")[0] == label && label == param2.split("_")[0]) {
                                            println("MSQL 180")
                                            if (param1.split("_")[1].contains(index) && param2.split("_")[1].contains(
                                                    index
                                                )
                                            ) {
                                                var operator = cond.operator
                                                var index1 = param1.split("_")[1]
                                                var index2 = param2.split("_")[1]

                                                if (index1 == index && index2 == "($index+1)") {
                                                    if (operator == ">=") {
                                                        return "SORT BY $field DESC\n"
                                                    }
                                                    if (operator == "<=") {
                                                        return "SORT BY $field ASC\n"
                                                    }
                                                }

                                                if (index1 == "($index+1)" && index2 == index) {
                                                    if (operator == ">=") {
                                                        return "SORT BY $field ASC\n"
                                                    }
                                                    if (operator == "<=") {
                                                        return "SORT BY $field DESC\n"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return ""
}

fun getLimit(setql: Array<Condition>): String {
    for (cond in setql) {
        if (arrayOf("=", "<=").contains(cond.operator) && cond.variable1 is Function) {
            if (cond.variable1.label == "abs" && cond.variable1.parameters.size == 1 && cond.variable1.parameters[0].label == "/Q") {
                return "LIMIT ${cond.variable2.label}"
            }
        }
    }

    return ""
}

fun generateMySql(setql: Array<Condition>): String {
    return """
SELECT ${getFields(setql)}
FROM ${getSet(setql)}${getGroupBy(setql)}${getConditions(setql)}${getSortByAndOrder(setql)}${getLimit(setql)}${"\n"}"""
}
