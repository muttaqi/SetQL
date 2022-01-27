package org.setql.server.syntax

class Member(label: String): Variable(label) {
    override fun toJSON(): String {
        return "{\"label\": \"${this.label}\"}"
    }
}