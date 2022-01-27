package org.setql.server.syntax

class Set(): Variable(null) {
    override var type = VariableType.Set

    var memberVariable: Variable? = null
    var memberCondition: Condition? = null

    lateinit var conditions: Array<Condition>

    constructor(memberVar: Variable, conds: Array<Condition>) : this() {
        this.memberVariable = memberVar
        this.conditions = conds
    }

    constructor(memberCond: Condition, conds: Array<Condition>) : this() {
        this.memberCondition = memberCond
        this.conditions = conds
    }

    override fun toJSON(): String {
        return "{\"member\": ${if (memberVariable != null) memberVariable!!.toJSON() else memberCondition!!.toJSON()}, \"conditions\": [${this.conditions.map {e -> e.toJSON() + ","}.joinToString()}]}"
    }
}