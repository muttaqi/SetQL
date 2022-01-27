package org.setql.server.syntax

open class Variable(val label: String?) {
    open var type = VariableType.Label

    open fun toJSON(): String {
        return "{\"label\": \"$label\"}"
    }
}

enum class VariableType {
    Function,
    Set,
    Label
}