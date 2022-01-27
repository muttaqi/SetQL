package org.setql.server

import io.ktor.application.Application
import io.ktor.application.*
import io.ktor.features.*
import io.ktor.routing.*
import io.ktor.serialization.*

import org.setql.server.routes.*

fun main(args: Array<String>): Unit = io.ktor.server.netty.EngineMain.main(args)

fun Application.module() {
    install(ContentNegotiation) {
        json()
    }
    install(CallLogging)
    registerQueryRoutes()
}

fun Application.registerQueryRoutes() {
    routing {
        queryRouting()
    }
}