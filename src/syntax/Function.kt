package org.setql.server.syntax

class Function(label: String, val parameters: Array<Variable>): Variable(label) {

    override var type = VariableType.Function

    override fun toJSON(): String {
        return "{\"label\": \"${this.label}\", \"parameters\": [${this.parameters.map {e -> e.toJSON()}.joinToString()}]}"
    }

    companion object {
        var fieldAggregators = arrayOf("max", "min", "/S")
        var fieldAggregatorToSQL = mapOf<String, String>("max" to "MAX", "min" to "MIN", "/S" to "SUM", "abs" to "COUNT")

        var setAggregators = arrayOf("abs")
        var setAggregatorToSQL = mapOf<String, String>("abs" to "COUNT")
    }
}