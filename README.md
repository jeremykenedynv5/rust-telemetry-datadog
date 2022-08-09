# Rust-telemetry-datadog

This project is a prototype to implement observervability in rust-actix-web framework. Traces are generated for each request and are stored in open-telemetry compatible services (Jaeger or Datadog).
## Prerequisites

1. Using Jaeger:

To execute this example with Jaeger you need a running Jaeger instance.  
You can launch one using Docker:

```bash
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
```

2. Using Datadog:

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