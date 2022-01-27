package org.setql.server.routes

import io.ktor.routing.*
import io.ktor.application.*
import io.ktor.http.*
import io.ktor.request.*
import io.ktor.response.*

import org.setql.operations.*

fun Route.queryRouting() {
    route("/query") {
        post {
            var query = call.receiveText()

            call.application.environment.log.info("query")
            println("query")

            var output = runQuery(query)

            call.respondText(output, status = HttpStatusCode.Accepted)
        }
    }
}