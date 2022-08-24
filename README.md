# Rust-telemetry-datadog

This project is a prototype to implement observervability in rust-actix-web framework. Traces are generated for each request and are stored in open-telemetry compatible services (Jaeger or Datadog).
## Prerequisites

1. Postgres Environment:
     
     We use psql to do health_check of our database. Check [these instructions](https://www.timescale.com/blog/how-to-install-psql-on-mac-ubuntu-debian-windows/) on how to install it on your OS.

     Run the following script to do database migration. The SKIP_DOCKER flag makes it easy to run migrations against an existing Postgres instance without having to tear it down manually and re-create it with scripts/init_db.sh.

     ```
     SKIP_DOCKER=true ./scripts/init_db.sh
     ```

     If you are curious to check the database using a GUI, you may install [Pg Admin](https://www.pgadmin.org/) or [Adminer](https://www.adminer.org/) and connect using the dafault database connection string used in this project (for development purpose only) DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/userdb.

2. Tracing Environment:    

    There are few incompatibilities between datadog and OTel. and see more details about it in the following links.
        1. https://docs.rs/opentelemetry-datadog/latest/opentelemetry_datadog/#quirks
        2. https://docs.datadoghq.com/tracing/other_telemetry/connect_logs_and_traces/opentelemetry
        3. https://github.com/open-telemetry/opentelemetry-rust/issues/820
        4. https://github.com/tokio-rs/tracing/issues/1531
    In order to circumvent the above issues, we send the traces to OTEL collector and use Datadog exporter to forword them to Datadog. More readings could be found in this link - https://docs.datadoghq.com/tracing/trace_collection/open_standards/otel_collector_datadog_exporter/

    Follow the following steps to setup your tracing environment

    1. Update datadog api key in otel_collector_config.yaml file in the root folder of this project
    2. Run the OTEL collector container using the below script

    HINT: make sure that your present working directory (pwd) is the root folder of this project

    ```
    docker run \
    -p 4317:4317 \
    --hostname $(hostname) \
    -v $(pwd)/otel_collector_config.yaml:/etc/otelcol-contrib/config.yaml \
    otel/opentelemetry-collector-contrib:latest
    ```
## Running

You can launch this example with 

```bash
cargo run
```

An `actix-web` application will be listening on port `8000`.  

You can fire the below request to it with and expect HTTP/1.1 200 OK as reponse:

```bash
curl -i http://localhost:8000
```
## Traces

Now insert a row to database using the belowe script

```
curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' http://localhost:8000/create_user
```

This will insert a new row in the database returns a `HTTP 200 Success` response. 

If you rerun the curl command, the query fails and you will get `HTTP 500 Internal Server Error` as response.

We can invetigate this failure using traces and spans from Datadog APM dashboard (Please wait for 30-60 sec before for data to show up in datadog dashboard) . Looking through the  structured log records we can understand that the database insert has failed with "duplicate key value violates unique constraint" error. 

## Credits
This application uses Open Source components. You can find the source code of their open source projects along with license information below. I acknowledge and am grateful to these developers for their contributions to open source.

```
 Project: https://github.com/LukeMathWalker/tracing-actix-web/tree/main/examples/opentelemetry
 Copyright (c) 2022 LukeMathWalker
 License (MIT) https://github.com/LukeMathWalker/tracing-actix-web/blob/main/LICENSE-MIT
```

## License

This program is licensed under the "MIT License". Please see the file LICENSE in the source distribution of this software for license terms.