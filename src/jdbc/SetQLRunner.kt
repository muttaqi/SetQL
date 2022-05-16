package org.setql.server.jdbc

import java.sql.*

import org.setql.operations.*

class SetQLRunner(val connection: Connection) {
    fun executeQuery(setql: String): ResultSet {
        var sql: String = sqlFromSetQLQuery(setql)
        var stmt = connection!!.createStatement()
        return stmt!!.executeQuery(sql)
    }
}