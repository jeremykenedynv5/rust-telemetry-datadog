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

    2.1 Using Jaeger:

    To execute this example with Jaeger you need a running Jaeger instance.  
    You can launch one using Docker:

    ```bash
    docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
    ```

    2.2 Using Datadog:

    To execute this example with Datadog, first run version 7.22.0 or above of the datadog-agent locally as described [here](https://docs.datadoghq.com/agent/)

    Hint: I used the following script to install dd-agent in Mac M1. You have to use your appropriate Datadog API key.

    ```
    DD_AGENT_MAJOR_VERSION=7 DD_API_KEY=<API-KEY-HERE> DD_SITE="datadoghq.com" bash -c "$(curl -L https://s3.amazonaws.com/dd-agent/scripts/install_mac_os.sh)"
    ```


## Running

You can launch this example with 

```bash
cargo run
```

An `actix-web` application will be listening on port `8000`.  
You can fire requests to it with:

```bash
curl -v http://localhost:8000/OptimusPrime
```
```text
Hello OptimusPrime!
```

## Traces

- If you have used Jaeger, you can look at the exported traces in your browser by visiting [http://localhost:16686](http://localhost:16686).  
Spans will be also printed to the console in JSON format, as structured log records.

- If you have used datadog, traces should appear in APM dashboard.

## Credits
This application uses Open Source components. You can find the source code of their open source projects along with license information below. I acknowledge and am grateful to these developers for their contributions to open source.

```
 Project: https://github.com/LukeMathWalker/tracing-actix-web/tree/main/examples/opentelemetry
 Copyright (c) 2022 LukeMathWalker
 License (MIT) https://github.com/LukeMathWalker/tracing-actix-web/blob/main/LICENSE-MIT
```

## License

This program is licensed under the "MIT License". Please see the file LICENSE in the source distribution of this software for license terms.