curl localhost:8080/query -X POST -d " /Q = {v /e S | f(v) = 2}"

curl localhost:8080/query -X POST -d " /Q /c {v /e S | f(v) = 2} ^ |/Q| = 1"

curl localhost:8080/query -X POST -d " /Q /c {v /e S | f(v) = 2} ^ |/Q| = n"

curl localhost:8080/query -X POST -d " /Q = {v_i /e S | f(v_i) >= f(v_i+1)} ^ |/Q| = n"

curl localhost:8080/query -X POST -d " /Q = {v | w /e S | f(v) = f(w) ^ g(v) = g(w) }"

curl localhost:8080/query -X POST -d " /Q = {v | max_f(v) = max({f(w) | w /e S ^ g(v) = g(w)})}"

curl localhost:8080/query -X POST -d " /Q = {v | sum_f(v) = /S({f(w) | w /e S ^ g(v) = g(w)})}"

curl localhost:8080/query -X POST -d " /Q = {v | count(v) = |{w /e S | g(v) = g(w) ^ h(w) < 100}|}"
